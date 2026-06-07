use bcrypt::{non_truncating_hash, BcryptError, DEFAULT_COST};
use chrono::NaiveDate;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use std::{error::Error, fmt};

use crate::models::authentication::Authentication;
use crate::models::user::{NewUser, User};

#[derive(Debug)]
pub enum CreateUserError {
    Database(DieselError),
    PasswordHash(BcryptError),
}

impl fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error creating user: {error}"),
            Self::PasswordHash(error) => write!(f, "password hashing failed: {error}"),
        }
    }
}

impl Error for CreateUserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::PasswordHash(error) => Some(error),
        }
    }
}

impl From<DieselError> for CreateUserError {
    fn from(error: DieselError) -> Self {
        Self::Database(error)
    }
}

impl From<BcryptError> for CreateUserError {
    fn from(error: BcryptError) -> Self {
        Self::PasswordHash(error)
    }
}

pub async fn create_user(
    conn: &mut AsyncPgConnection,
    name: &str,
    email: &str,
    date_of_birth: Option<NaiveDate>,
    password: &str,
) -> Result<User, CreateUserError> {
    let hashed_password = non_truncating_hash(password, DEFAULT_COST)?;
    let new_user = NewUser {
        name: name.to_string(),
        email: email.to_string(),
        date_of_birth,
        created_at: chrono::Utc::now().naive_utc(),
        kyc_verified: false,
        email_verified: true,
    };

    conn.transaction::<_, CreateUserError, _>(|conn| {
        Box::pin(async move {
            let inserted_user = User::create(new_user, conn).await?;
            let new_auth = Authentication {
                user_id: inserted_user.id(),
                type_authentication: "password".to_string(),
                info_auth: hashed_password,
            };

            Authentication::create(new_auth, conn).await?;

            Ok(inserted_user)
        })
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::CreateUserError;
    use bcrypt::BcryptError;

    #[test]
    fn preserves_password_hashing_errors() {
        let error = CreateUserError::from(BcryptError::Truncation(72));

        assert!(matches!(error, CreateUserError::PasswordHash(_)));
        assert!(error.to_string().contains("password hashing failed"));
    }
}
