use diesel::prelude::*;
use crate::db::schema::users;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<NaiveDate>, // Use Option if the field can be null
    pub created_at: NaiveDateTime,
    pub kyc_verified: bool,
}

impl User {
    // Method to get the user's id
    pub fn id(&self) -> i32 {
        self.id
    }
}