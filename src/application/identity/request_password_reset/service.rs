use futures::future::BoxFuture;

use super::{RequestPasswordResetCommand, RequestPasswordResetError, RequestPasswordResetOutcome};

pub trait RequestPasswordResetUseCase: Send + Sync {
    fn request_password_reset(
        &self,
        command: RequestPasswordResetCommand,
    ) -> BoxFuture<'_, Result<RequestPasswordResetOutcome, RequestPasswordResetError>>;
}
