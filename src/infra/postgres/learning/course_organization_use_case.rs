use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::list_course_organizations::{
    self, CourseOrganizationOutput, CourseOrganizationReadError, CourseOrganizationsUseCase,
};
use crate::infra::postgres::learning::course_organization_store::PostgresCourseOrganizationStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresCourseOrganizationsUseCase {
    pool: DbPool,
}

impl PostgresCourseOrganizationsUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseOrganizationsUseCase for PostgresCourseOrganizationsUseCase {
    fn list_course_organizations(
        &self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseOrganizationOutput>, CourseOrganizationReadError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseOrganizationStore::new(&mut conn);
            list_course_organizations::list_course_organizations(&mut store, course_id).await
        }
        .boxed()
    }
}

impl PostgresCourseOrganizationsUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseOrganizationReadError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseOrganizationReadError::Connection(error.to_string()))
    }
}
