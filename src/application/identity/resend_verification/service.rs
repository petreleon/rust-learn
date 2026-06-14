use futures::future::BoxFuture;

use super::{ResendVerificationCommand, ResendVerificationError, ResendVerificationOutcome};

pub trait ResendVerificationUseCase: Send + Sync {
    fn resend_verification(
        &self,
        command: ResendVerificationCommand,
    ) -> BoxFuture<'_, Result<ResendVerificationOutcome, ResendVerificationError>>;
}
