use crate::db::schema::users;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<NaiveDate>, // Use Option if the field can be null
    pub created_at: NaiveDateTime,
    pub kyc_verified: bool,
    pub email_verified: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub kyc_verified: bool,
    pub email_verified: bool,
}

impl User {
    pub fn id(&self) -> i32 {
        self.id
    }
}
