use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::AsyncConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::register::{
    RegisterError, RegisterStore, RegisteredUser, RegistrationAccount,
};
use crate::infra::postgres::identity::accounts::{
    create_unverified_student_password_account, NewIdentityPasswordAccount,
};
use crate::infra::postgres::identity::email_verification_tokens::create_email_verification_token;
use crate::infra::tokens::identity::identity_token_hash;

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
            let new_account = NewIdentityPasswordAccount {
                name: account.name,
                email: account.email,
                date_of_birth: account.date_of_birth,
                password_hash,
            };
            let token_hash = identity_token_hash(&verification_token);

            self.conn
                .transaction::<_, DieselError, _>(|conn| {
                    Box::pin(async move {
                        let inserted_user =
                            create_unverified_student_password_account(conn, new_account).await?;

                        create_email_verification_token(conn, inserted_user.user_id, token_hash)
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
