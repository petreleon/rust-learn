use futures::future::BoxFuture;

use super::{VerifyEmailCommand, VerifyEmailError, VerifyEmailOutcome};

pub trait VerifyEmailUseCase: Send + Sync {
    fn verify_email(
        &self,
        command: VerifyEmailCommand,
    ) -> BoxFuture<'_, Result<VerifyEmailOutcome, VerifyEmailError>>;
}
