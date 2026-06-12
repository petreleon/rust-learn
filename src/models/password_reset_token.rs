use crate::db::schema::password_reset_tokens;
use crate::models::user::User;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

const PASSWORD_RESET_TOKEN_TTL_HOURS: i64 = 1;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = password_reset_tokens)]
pub struct PasswordResetToken {
    pub id: i32,
    pub user_id: i32,
    pub token_hash: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = password_reset_tokens)]
pub struct NewPasswordResetToken {
    pub user_id: i32,
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordResetResult {
    Reset { user_id: i32 },
    Expired,
    Invalid,
}

impl PasswordResetToken {
    pub async fn create_for_user(
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

    pub async fn consume(
        conn: &mut AsyncPgConnection,
        token_hash: &str,
    ) -> QueryResult<PasswordResetResult> {
        use crate::db::schema::password_reset_tokens::dsl as tokens;

        let now = Utc::now().naive_utc();
        let token = tokens::password_reset_tokens
            .filter(tokens::token_hash.eq(token_hash))
            .first::<PasswordResetToken>(conn)
            .await
            .optional()?;

        let Some(token) = token else {
            return Ok(PasswordResetResult::Invalid);
        };

        if token.used_at.is_some() {
            return Ok(PasswordResetResult::Invalid);
        }

        if token.expires_at <= now {
            return Ok(PasswordResetResult::Expired);
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

        Ok(PasswordResetResult::Reset {
            user_id: token.user_id,
        })
    }
}
