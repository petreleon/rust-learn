use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{email_verification_tokens, users};
use crate::models::email_verification_token::{EmailVerificationToken, NewEmailVerificationToken};
use crate::models::user::User;

const EMAIL_VERIFICATION_TOKEN_TTL_HOURS: i64 = 24;

#[derive(Debug, PartialEq, Eq)]
pub enum EmailVerificationTokenStatus {
    Verified,
    AlreadyVerified,
    Expired,
    Invalid,
}

pub async fn create_email_verification_token(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    token_hash: String,
) -> QueryResult<usize> {
    use crate::db::schema::email_verification_tokens::dsl as tokens;

    let now = Utc::now().naive_utc();

    diesel::update(
        tokens::email_verification_tokens
            .filter(tokens::user_id.eq(user_id))
            .filter(tokens::used_at.is_null()),
    )
    .set(tokens::used_at.eq(now))
    .execute(conn)
    .await?;

    let new_token = NewEmailVerificationToken {
        user_id,
        token_hash,
        expires_at: now + Duration::hours(EMAIL_VERIFICATION_TOKEN_TTL_HOURS),
    };

    diesel::insert_into(email_verification_tokens::table)
        .values(&new_token)
        .execute(conn)
        .await
}

pub async fn verify_email_verification_token(
    conn: &mut AsyncPgConnection,
    token_hash: &str,
) -> QueryResult<EmailVerificationTokenStatus> {
    use crate::db::schema::email_verification_tokens::dsl as tokens;

    let now = Utc::now().naive_utc();
    let token = tokens::email_verification_tokens
        .filter(tokens::token_hash.eq(token_hash))
        .first::<EmailVerificationToken>(conn)
        .await
        .optional()?;

    let Some(token) = token else {
        return Ok(EmailVerificationTokenStatus::Invalid);
    };

    let user = users::table.find(token.user_id).first::<User>(conn).await?;
    if user.email_verified {
        return Ok(EmailVerificationTokenStatus::AlreadyVerified);
    }

    if token.used_at.is_some() {
        return Ok(EmailVerificationTokenStatus::Invalid);
    }

    if token.expires_at <= now {
        return Ok(EmailVerificationTokenStatus::Expired);
    }

    diesel::update(users::table.find(token.user_id))
        .set(users::email_verified.eq(true))
        .execute(conn)
        .await?;

    diesel::update(tokens::email_verification_tokens.find(token.id))
        .set(tokens::used_at.eq(now))
        .execute(conn)
        .await?;

    Ok(EmailVerificationTokenStatus::Verified)
}
