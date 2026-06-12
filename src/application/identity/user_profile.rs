use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfileOutput {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub kyc_verified: bool,
    pub email_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserProfileError {
    NotFound,
    Database(String),
}
