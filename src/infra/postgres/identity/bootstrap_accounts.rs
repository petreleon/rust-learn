use bcrypt::{non_truncating_hash, BcryptError, DEFAULT_COST};
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use std::{error::Error, fmt};

use crate::db::schema::{authentications, users};

const PASSWORD_AUTH_TYPE: &str = "password";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPasswordAccount {
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<NaiveDate>,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedBootstrapAccount {
    pub user_id: i32,
    pub name: String,
}

#[derive(Debug)]
pub enum BootstrapAccountError {
    Database(DieselError),
    PasswordHash(BcryptError),
}

impl fmt::Display for BootstrapAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => {
                write!(f, "database error creating bootstrap account: {error}")
            }
            Self::PasswordHash(error) => write!(f, "bootstrap password hashing failed: {error}"),
        }
    }
}

impl Error for BootstrapAccountError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::PasswordHash(error) => Some(error),
        }
    }
}

impl From<DieselError> for BootstrapAccountError {
    fn from(error: DieselError) -> Self {
        Self::Database(error)
    }
}

impl From<BcryptError> for BootstrapAccountError {
    fn from(error: BcryptError) -> Self {
        Self::PasswordHash(error)
    }
}

pub async fn create_verified_password_account(
    conn: &mut AsyncPgConnection,
    account: BootstrapPasswordAccount,
) -> Result<CreatedBootstrapAccount, BootstrapAccountError> {
    let password_hash = non_truncating_hash(&account.password, DEFAULT_COST)?;

    conn.transaction::<_, BootstrapAccountError, _>(|conn| {
        Box::pin(async move {
            let (user_id, name) = diesel::insert_into(users::table)
                .values((
                    users::name.eq(account.name),
                    users::email.eq(account.email),
                    users::date_of_birth.eq(account.date_of_birth),
                    users::created_at.eq(Utc::now().naive_utc()),
                    users::kyc_verified.eq(false),
                    users::email_verified.eq(true),
                ))
                .returning((users::id, users::name))
                .get_result::<(i32, String)>(conn)
                .await?;

            diesel::insert_into(authentications::table)
                .values((
                    authentications::user_id.eq(user_id),
                    authentications::type_authentication.eq(PASSWORD_AUTH_TYPE),
                    authentications::info_auth.eq(Some(password_hash)),
                ))
                .execute(conn)
                .await?;

            Ok(CreatedBootstrapAccount { user_id, name })
        })
    })
    .await
}
