use futures::future::BoxFuture;

use super::{ResetPasswordCommand, ResetPasswordError, ResetPasswordOutcome};

pub trait ResetPasswordUseCase: Send + Sync {
    fn reset_password(
        &self,
        command: ResetPasswordCommand,
    ) -> BoxFuture<'_, Result<ResetPasswordOutcome, ResetPasswordError>>;
}
