use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{authentications, users};

const PASSWORD_AUTH_TYPE: &str = "password";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityUserAccount {
    pub user_id: i32,
    pub email: String,
    pub name: String,
    pub email_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordAuthenticationAccount {
    pub user_id: i32,
    pub email_verified: bool,
    pub password_hash: Option<String>,
}

pub async fn find_identity_user_by_email(
    conn: &mut AsyncPgConnection,
    email: &str,
) -> QueryResult<Option<IdentityUserAccount>> {
    users::table
        .filter(users::email.eq(email))
        .select((users::id, users::email, users::name, users::email_verified))
        .first::<(i32, String, String, bool)>(conn)
        .await
        .optional()
        .map(|user| user.map(map_identity_user_account))
}

pub async fn find_password_authentication_by_email(
    conn: &mut AsyncPgConnection,
    email: &str,
) -> QueryResult<Option<PasswordAuthenticationAccount>> {
    users::table
        .filter(users::email.eq(email))
        .inner_join(authentications::table.on(users::id.eq(authentications::user_id)))
        .filter(authentications::type_authentication.eq(PASSWORD_AUTH_TYPE))
        .select((
            users::id,
            users::email_verified,
            authentications::info_auth.nullable(),
        ))
        .first::<(i32, bool, Option<String>)>(conn)
        .await
        .optional()
        .map(|account| account.map(map_password_authentication_account))
}

fn map_identity_user_account(
    (user_id, email, name, email_verified): (i32, String, String, bool),
) -> IdentityUserAccount {
    IdentityUserAccount {
        user_id,
        email,
        name,
        email_verified,
    }
}

fn map_password_authentication_account(
    (user_id, email_verified, password_hash): (i32, bool, Option<String>),
) -> PasswordAuthenticationAccount {
    PasswordAuthenticationAccount {
        user_id,
        email_verified,
        password_hash,
    }
}
