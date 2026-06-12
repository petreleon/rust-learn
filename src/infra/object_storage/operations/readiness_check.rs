use futures::future::{BoxFuture, FutureExt};

use crate::application::operations::ports::ReadinessDependency;
use crate::utils::s3_utils::S3State;

pub struct S3ReadinessCheck {
    state: S3State,
}

impl S3ReadinessCheck {
    pub fn new(state: S3State) -> Self {
        Self { state }
    }
}

impl ReadinessDependency for S3ReadinessCheck {
    fn name(&self) -> &'static str {
        "s3"
    }

    fn check(&mut self) -> BoxFuture<'_, Result<(), String>> {
        let state = self.state.clone();
        async move { state.health_check().await.map_err(|err| err.to_string()) }.boxed()
    }
}
