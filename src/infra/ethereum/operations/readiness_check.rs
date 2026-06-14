use ethers::providers::Middleware;
use futures::future::{BoxFuture, FutureExt};

use crate::application::operations::ports::ReadinessDependency;
use crate::infra::ethereum::operations::provider::try_get_provider;

pub struct EthereumReadinessCheck;

impl ReadinessDependency for EthereumReadinessCheck {
    fn name(&self) -> &'static str {
        "ethereum"
    }

    fn check(&mut self) -> BoxFuture<'_, Result<(), String>> {
        async move {
            let provider = try_get_provider()?;
            provider
                .get_chainid()
                .await
                .map(|_| ())
                .map_err(|err| err.to_string())
        }
        .boxed()
    }
}
