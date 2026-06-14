use futures::future::BoxFuture;

use crate::application::operations::readiness_check::ReadinessOutput;

pub trait ReadinessUseCase: Send + Sync {
    fn check(&self) -> BoxFuture<'_, ReadinessOutput>;
}
