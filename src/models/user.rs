use diesel::prelude::*;
use crate::db::schema::users;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    date_of_birth: Option<NaiveDate>, // Use Option if the field can be null
    created_at: NaiveDateTime,
    kyc_verified: bool,
}
