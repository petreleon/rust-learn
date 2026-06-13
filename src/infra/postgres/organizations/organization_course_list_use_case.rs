use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::list_organization_courses::{
    self, OrganizationCourseListError, OrganizationCourseListOutput, OrganizationCourseListQuery,
    OrganizationCourseListUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::organizations::organization_course_list_store::PostgresOrganizationCourseListStore;

#[derive(Clone)]
pub struct PostgresOrganizationCourseListUseCase {
    pool: DbPool,
}

impl PostgresOrganizationCourseListUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationCourseListUseCase for PostgresOrganizationCourseListUseCase {
    fn list_organization_courses(
        &self,
        query: OrganizationCourseListQuery,
    ) -> BoxFuture<'_, Result<OrganizationCourseListOutput, OrganizationCourseListError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationCourseListStore::new(&mut conn);
            list_organization_courses::list_organization_courses(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresOrganizationCourseListUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, OrganizationCourseListError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationCourseListError::Connection(error.to_string()))
    }
}
