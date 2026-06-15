use crate::db::schema::email_verification_tokens;
use crate::infra::postgres::models::user::User;
use chrono::NaiveDateTime;
use diesel::prelude::*;

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
