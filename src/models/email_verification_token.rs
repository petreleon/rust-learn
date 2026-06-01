use crate::db::schema::{email_verification_tokens, users};
use crate::models::user::User;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

const EMAIL_VERIFICATION_TOKEN_TTL_HOURS: i64 = 24;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = email_verification_tokens)]
pub struct EmailVerificationToken {
    pub id: i32,
    pub user_id: i32,
    pub token_hash: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = email_verification_tokens)]
pub struct NewEmailVerificationToken {
    pub user_id: i32,
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
}

impl EmailVerificationToken {
    pub async fn create_for_user(
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

    pub async fn verify(conn: &mut AsyncPgConnection, token_hash: &str) -> QueryResult<bool> {
        use crate::db::schema::email_verification_tokens::dsl as tokens;

        let now = Utc::now().naive_utc();
        let token = tokens::email_verification_tokens
            .filter(tokens::token_hash.eq(token_hash))
            .filter(tokens::used_at.is_null())
            .filter(tokens::expires_at.gt(now))
            .first::<EmailVerificationToken>(conn)
            .await
            .optional()?;

        let Some(token) = token else {
            return Ok(false);
        };

        diesel::update(users::table.find(token.user_id))
            .set(users::email_verified.eq(true))
            .execute(conn)
            .await?;

        diesel::update(tokens::email_verification_tokens.find(token.id))
            .set(tokens::used_at.eq(now))
            .execute(conn)
            .await?;

        Ok(true)
    }
}
