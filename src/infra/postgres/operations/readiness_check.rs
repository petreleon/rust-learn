use diesel::sql_types::Integer;
use diesel_async::RunQueryDsl;
use futures::future::{BoxFuture, FutureExt};

use crate::application::operations::ports::{ReadinessDependency, READINESS_DEPENDENCY_DATABASE};
use crate::db::DbPool;

pub struct PostgresReadinessCheck {
    pool: DbPool,
}

impl PostgresReadinessCheck {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ReadinessDependency for PostgresReadinessCheck {
    fn name(&self) -> &'static str {
        READINESS_DEPENDENCY_DATABASE
    }

    fn check(&mut self) -> BoxFuture<'_, Result<(), String>> {
        let pool = self.pool.clone();
        async move {
            let mut conn = pool
                .get()
                .await
                .map_err(|err| format!("database pool checkout failed: {err}"))?;
            let _: i32 = diesel::select(diesel::dsl::sql::<Integer>("1"))
                .get_result(&mut conn)
                .await
                .map_err(|err| err.to_string())?;
            Ok(())
        }
        .boxed()
    }
}
