use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::password_reset_tokens;
use crate::models::password_reset_token::{NewPasswordResetToken, PasswordResetToken};

const PASSWORD_RESET_TOKEN_TTL_HOURS: i64 = 1;

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordResetTokenStatus {
    Reset { user_id: i32 },
    Expired,
    Invalid,
}

pub async fn create_password_reset_token(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    token_hash: String,
) -> QueryResult<usize> {
    use crate::db::schema::password_reset_tokens::dsl as tokens;

    let now = Utc::now().naive_utc();

    diesel::update(
        tokens::password_reset_tokens
            .filter(tokens::user_id.eq(user_id))
            .filter(tokens::used_at.is_null()),
    )
    .set(tokens::used_at.eq(now))
    .execute(conn)
    .await?;

    let new_token = NewPasswordResetToken {
        user_id,
        token_hash,
        expires_at: now + Duration::hours(PASSWORD_RESET_TOKEN_TTL_HOURS),
    };

    diesel::insert_into(password_reset_tokens::table)
        .values(&new_token)
        .execute(conn)
        .await
}

pub async fn consume_password_reset_token(
    conn: &mut AsyncPgConnection,
    token_hash: &str,
) -> QueryResult<PasswordResetTokenStatus> {
    use crate::db::schema::password_reset_tokens::dsl as tokens;

    let now = Utc::now().naive_utc();
    let token = tokens::password_reset_tokens
        .filter(tokens::token_hash.eq(token_hash))
        .first::<PasswordResetToken>(conn)
        .await
        .optional()?;

    let Some(token) = token else {
        return Ok(PasswordResetTokenStatus::Invalid);
    };

    if token.used_at.is_some() {
        return Ok(PasswordResetTokenStatus::Invalid);
    }

    if token.expires_at <= now {
        return Ok(PasswordResetTokenStatus::Expired);
    }

    diesel::update(tokens::password_reset_tokens.find(token.id))
        .set(tokens::used_at.eq(now))
        .execute(conn)
        .await?;

    diesel::update(
        tokens::password_reset_tokens
            .filter(tokens::user_id.eq(token.user_id))
            .filter(tokens::used_at.is_null()),
    )
    .set(tokens::used_at.eq(now))
    .execute(conn)
    .await?;

    Ok(PasswordResetTokenStatus::Reset {
        user_id: token.user_id,
    })
}
