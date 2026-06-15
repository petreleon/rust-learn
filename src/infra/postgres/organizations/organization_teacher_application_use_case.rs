use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::list_organization_teacher_applications::{
    self, OrganizationTeacherApplicationListError, OrganizationTeacherApplicationListOutput,
    OrganizationTeacherApplicationListQuery, OrganizationTeacherApplicationListUseCase,
};
use crate::infra::postgres::organizations::organization_teacher_application_store::PostgresOrganizationTeacherApplicationStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresOrganizationTeacherApplicationUseCase {
    pool: DbPool,
}

impl PostgresOrganizationTeacherApplicationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationTeacherApplicationListUseCase for PostgresOrganizationTeacherApplicationUseCase {
    fn list_organization_teacher_applications(
        &self,
        query: OrganizationTeacherApplicationListQuery,
    ) -> BoxFuture<
        '_,
        Result<OrganizationTeacherApplicationListOutput, OrganizationTeacherApplicationListError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationTeacherApplicationStore::new(&mut conn);
            list_organization_teacher_applications::list_organization_teacher_applications(
                &mut store, query,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresOrganizationTeacherApplicationUseCase {
    async fn connection(
        &self,
    ) -> Result<Object<AsyncPgConnection>, OrganizationTeacherApplicationListError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationTeacherApplicationListError::Connection(error.to_string()))
    }
}
