use futures::future::BoxFuture;

use super::{LoginCommand, LoginError, LoginOutput};

pub trait LoginUseCase: Send + Sync {
    fn login(&self, command: LoginCommand) -> BoxFuture<'_, Result<LoginOutput, LoginError>>;
}
