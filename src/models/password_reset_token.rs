use crate::db::schema::password_reset_tokens;
use crate::models::user::User;
use chrono::NaiveDateTime;
use diesel::prelude::*;

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
