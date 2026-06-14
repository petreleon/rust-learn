use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::reset_password::{
    ResetPasswordError, ResetPasswordOutcome, ResetPasswordStore,
};
use crate::infra::postgres::identity::password_reset_tokens::{
    consume_password_reset_token, PasswordResetTokenStatus,
};
use crate::infra::tokens::identity::identity_token_hash;

const PASSWORD_AUTH_TYPE: &str = "password";

pub struct PostgresResetPasswordStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
}

impl<'conn> PostgresResetPasswordStore<'conn> {
    pub fn new(conn: &'conn mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl ResetPasswordStore for PostgresResetPasswordStore<'_> {
    fn reset_password(
        &mut self,
        token: String,
        password_hash: String,
    ) -> BoxFuture<'_, Result<ResetPasswordOutcome, ResetPasswordError>> {
        async move {
            let token_hash = identity_token_hash(&token);
            self.conn
                .transaction::<_, DieselError, _>(|conn| {
                    Box::pin(async move {
                        let outcome = consume_password_reset_token(conn, &token_hash).await?;
                        let PasswordResetTokenStatus::Reset { user_id } = outcome else {
                            return Ok(map_outcome(outcome));
                        };

                        use crate::db::schema::authentications::dsl as auths;
                        let updated = diesel::update(
                            auths::authentications
                                .filter(auths::user_id.eq(user_id))
                                .filter(auths::type_authentication.eq(PASSWORD_AUTH_TYPE)),
                        )
                        .set(auths::info_auth.eq(Some(password_hash)))
                        .execute(conn)
                        .await?;
                        if updated == 0 {
                            return Err(DieselError::NotFound);
                        }
                        Ok(ResetPasswordOutcome::Reset)
                    })
                })
                .await
                .map_err(|error| ResetPasswordError::Store(error.to_string()))
        }
        .boxed()
    }
}

fn map_outcome(outcome: PasswordResetTokenStatus) -> ResetPasswordOutcome {
    match outcome {
        PasswordResetTokenStatus::Reset { .. } => ResetPasswordOutcome::Reset,
        PasswordResetTokenStatus::Expired => ResetPasswordOutcome::Expired,
        PasswordResetTokenStatus::Invalid => ResetPasswordOutcome::Invalid,
    }
}
