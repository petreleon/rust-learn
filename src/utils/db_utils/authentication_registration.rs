// src/utils/db_utils/authentication_registration.rs

use diesel::prelude::*;
use diesel_async::AsyncPgConnection;

use crate::models::user::{User, NewUser};
use crate::models::authentication::Authentication;
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDate;

pub async fn create_user(
    conn: &mut AsyncPgConnection,
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

    let inserted_user = User::create(new_user, conn).await?;

    let hashed_password = hash(password, DEFAULT_COST).expect("Error hashing password");
    let new_auth = Authentication {
        user_id: inserted_user.id(),
        type_authentication: "password".to_string(),
        info_auth: hashed_password,
    };

    Authentication::create(new_auth, conn).await?;

    Ok(inserted_user)
}


// You might also include additional utility functions as needed for authentication, such as checking if an email already exists, etc.
