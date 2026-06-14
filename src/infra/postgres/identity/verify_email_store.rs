use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::verify_email::{
    VerifyEmailError, VerifyEmailOutcome, VerifyEmailStore,
};
use crate::models::email_verification_token::{EmailVerificationResult, EmailVerificationToken};
use crate::utils::email::verification_token_hash;

pub struct PostgresVerifyEmailStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
}

impl<'conn> PostgresVerifyEmailStore<'conn> {
    pub fn new(conn: &'conn mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl VerifyEmailStore for PostgresVerifyEmailStore<'_> {
    fn verify_email_token(
        &mut self,
        token: String,
    ) -> BoxFuture<'_, Result<VerifyEmailOutcome, VerifyEmailError>> {
        async move {
            let token_hash = verification_token_hash(&token);
            EmailVerificationToken::verify(self.conn, &token_hash)
                .await
                .map(map_outcome)
                .map_err(|error| VerifyEmailError::Database(error.to_string()))
        }
        .boxed()
    }
}

fn map_outcome(outcome: EmailVerificationResult) -> VerifyEmailOutcome {
    match outcome {
        EmailVerificationResult::Verified => VerifyEmailOutcome::Verified,
        EmailVerificationResult::AlreadyVerified => VerifyEmailOutcome::AlreadyVerified,
        EmailVerificationResult::Expired => VerifyEmailOutcome::Expired,
        EmailVerificationResult::Invalid => VerifyEmailOutcome::Invalid,
    }
}
