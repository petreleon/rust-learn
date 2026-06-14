use chrono::Utc;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::AsyncConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::register::{
    RegisterError, RegisterStore, RegisteredUser, RegistrationAccount,
};
use crate::models::authentication::Authentication;
use crate::models::email_verification_token::EmailVerificationToken;
use crate::models::role::PlatformRole;
use crate::models::user::{NewUser, User};
use crate::models::user_role_platform::UserRolePlatform;
use crate::utils::email::verification_token_hash;

const DEFAULT_REGISTRATION_ROLE: &str = "STUDENT";
const PASSWORD_AUTH_TYPE: &str = "password";

pub struct PostgresRegistrationStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
}

impl<'conn> PostgresRegistrationStore<'conn> {
    pub fn new(conn: &'conn mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RegisterStore for PostgresRegistrationStore<'_> {
    fn register_account(
        &mut self,
        account: RegistrationAccount,
        password_hash: String,
        verification_token: String,
    ) -> BoxFuture<'_, Result<RegisteredUser, RegisterError>> {
        async move {
            let new_user = NewUser {
                name: account.name,
                email: account.email,
                date_of_birth: account.date_of_birth,
                created_at: Utc::now().naive_utc(),
                kyc_verified: false,
                email_verified: false,
            };
            let token_hash = verification_token_hash(&verification_token);

            self.conn
                .transaction::<_, DieselError, _>(|conn| {
                    Box::pin(async move {
                        let inserted_user = User::create(new_user, conn).await?;
                        let role_id =
                            PlatformRole::find_by_name(DEFAULT_REGISTRATION_ROLE, conn).await?;
                        UserRolePlatform::assign(conn, inserted_user.id(), role_id).await?;

                        Authentication::create(
                            Authentication {
                                user_id: inserted_user.id(),
                                type_authentication: PASSWORD_AUTH_TYPE.to_string(),
                                info_auth: password_hash,
                            },
                            conn,
                        )
                        .await?;

                        EmailVerificationToken::create_for_user(
                            conn,
                            inserted_user.id(),
                            token_hash,
                        )
                        .await?;

                        Ok(RegisteredUser {
                            email: inserted_user.email,
                            name: inserted_user.name,
                        })
                    })
                })
                .await
                .map_err(map_registration_error)
        }
        .boxed()
    }
}

fn map_registration_error(error: DieselError) -> RegisterError {
    match error {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            RegisterError::EmailAlreadyRegistered
        }
        DieselError::NotFound => RegisterError::DefaultStudentRoleMissing,
        error => RegisterError::Store(error.to_string()),
    }
}
