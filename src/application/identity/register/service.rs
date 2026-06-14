use futures::future::BoxFuture;

use super::{RegisterCommand, RegisterError, RegisterOutcome};

pub trait RegisterUseCase: Send + Sync {
    fn register(
        &self,
        command: RegisterCommand,
    ) -> BoxFuture<'_, Result<RegisterOutcome, RegisterError>>;
}
