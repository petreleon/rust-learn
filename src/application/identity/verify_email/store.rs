use futures::future::BoxFuture;

use super::{VerifyEmailError, VerifyEmailOutcome};

pub trait VerifyEmailStore {
    fn verify_email_token(
        &mut self,
        token: String,
    ) -> BoxFuture<'_, Result<VerifyEmailOutcome, VerifyEmailError>>;
}
