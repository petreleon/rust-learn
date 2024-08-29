// src/utils/db_utils/authentication_registration.rs

use bcrypt::verify;
use diesel::prelude::*;
use diesel::PgConnection;
use crate::api::authentication::LoginQueryResult;
use crate::db::schema::{users, authentications};
use crate::models::{user::User, authentication::Authentication};
use crate::api::authentication::NewUser;
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDate;

pub fn create_user(
    conn: &mut PgConnection,
    name: &str,
    email: &str,
    date_of_birth: Option<NaiveDate>,
    password: &str,
) -> QueryResult<User> {
    let new_user = NewUser {
        name: name.to_string(),
        email: email.to_string(),
        date_of_birth,
        created_at: chrono::Utc::now().naive_utc(),
        kyc_verified: false,
    };

    let inserted_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn)?;

    let hashed_password = hash(password, DEFAULT_COST).expect("Error hashing password");
    let new_auth = Authentication {
        user_id: inserted_user.id(),
        type_authentication: "password".to_string(),
        info_auth: hashed_password,
    };

    diesel::insert_into(authentications::table)
        .values(&new_auth)
        .execute(conn)?;

    Ok(inserted_user)
}


// You might also include additional utility functions as needed for authentication, such as checking if an email already exists, etc.
