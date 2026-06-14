use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    list_application_audit::{
        self, TeacherApplicationAuditError, TeacherApplicationAuditQuery,
        TeacherApplicationAuditUseCase,
    },
    TeacherApplicationAuditEventOutput,
};
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_audit_store::PostgresTeacherApplicationAuditStore;

#[derive(Clone)]
pub struct PostgresTeacherApplicationAuditUseCase {
    pool: DbPool,
}

impl PostgresTeacherApplicationAuditUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherApplicationAuditUseCase for PostgresTeacherApplicationAuditUseCase {
    fn list_application_audit(
        &self,
        query: TeacherApplicationAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherApplicationAuditStore::new(&mut conn);
            list_application_audit::list_application_audit(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresTeacherApplicationAuditUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TeacherApplicationAuditError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherApplicationAuditError::Connection(error.to_string()))
    }
}
