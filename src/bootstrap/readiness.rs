use futures::future::{BoxFuture, FutureExt};

use crate::application::operations::ports::ReadinessDependency;
use crate::application::operations::readiness_check::{
    check_readiness, ReadinessOutput, ReadinessUseCase,
};
use crate::db::DbPool;
use crate::infra::ethereum::operations::readiness_check::EthereumReadinessCheck;
use crate::infra::object_storage::operations::readiness_check::S3ReadinessCheck;
use crate::infra::postgres::operations::readiness_check::PostgresReadinessCheck;
use crate::utils::s3_utils::S3State;

#[derive(Clone)]
pub struct RuntimeReadinessUseCase {
    pool: DbPool,
    s3: S3State,
}

impl RuntimeReadinessUseCase {
    pub fn new(pool: DbPool, s3: S3State) -> Self {
        Self { pool, s3 }
    }
}

impl ReadinessUseCase for RuntimeReadinessUseCase {
    fn check(&self) -> BoxFuture<'_, ReadinessOutput> {
        async move {
            let mut dependencies: Vec<Box<dyn ReadinessDependency>> = vec![
                Box::new(PostgresReadinessCheck::new(self.pool.clone())),
                Box::new(S3ReadinessCheck::new(self.s3.clone())),
                Box::new(EthereumReadinessCheck),
            ];

            check_readiness(&mut dependencies).await
        }
        .boxed()
    }
}
