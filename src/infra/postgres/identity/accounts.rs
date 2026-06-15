use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{authentications, platform_roles, user_role_platform, users};
use crate::infra::postgres::models::user::User;

const DEFAULT_REGISTRATION_ROLE: &str = "STUDENT";
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewIdentityPasswordAccount {
    pub email: String,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub password_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedIdentityAccount {
    pub user_id: i32,
    pub email: String,
    pub name: String,
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

pub async fn find_user_by_id(conn: &mut AsyncPgConnection, user_id: i32) -> QueryResult<User> {
    users::table.find(user_id).first(conn).await
}

pub async fn find_user_by_email(conn: &mut AsyncPgConnection, email: &str) -> QueryResult<User> {
    users::table
        .filter(users::email.eq(email))
        .first(conn)
        .await
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

pub async fn create_unverified_student_password_account(
    conn: &mut AsyncPgConnection,
    account: NewIdentityPasswordAccount,
) -> QueryResult<CreatedIdentityAccount> {
    let NewIdentityPasswordAccount {
        email,
        name,
        date_of_birth,
        password_hash,
    } = account;

    let (user_id, email, name) = diesel::insert_into(users::table)
        .values((
            users::name.eq(name),
            users::email.eq(email),
            users::date_of_birth.eq(date_of_birth),
            users::created_at.eq(Utc::now().naive_utc()),
            users::kyc_verified.eq(false),
            users::email_verified.eq(false),
        ))
        .returning((users::id, users::email, users::name))
        .get_result::<(i32, String, String)>(conn)
        .await?;

    let role_id = platform_roles::table
        .filter(platform_roles::name.eq(DEFAULT_REGISTRATION_ROLE))
        .select(platform_roles::id)
        .first::<i32>(conn)
        .await?;

    diesel::insert_into(user_role_platform::table)
        .values((
            user_role_platform::user_id.eq(Some(user_id)),
            user_role_platform::platform_role_id.eq(Some(role_id)),
        ))
        .execute(conn)
        .await?;

    diesel::insert_into(authentications::table)
        .values((
            authentications::user_id.eq(user_id),
            authentications::type_authentication.eq(PASSWORD_AUTH_TYPE),
            authentications::info_auth.eq(Some(password_hash)),
        ))
        .execute(conn)
        .await?;

    Ok(CreatedIdentityAccount {
        user_id,
        email,
        name,
    })
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
