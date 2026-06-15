use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::list_teacher_course_dashboard::{
    self, TeacherCourseDashboardListOutput, TeacherCourseDashboardListQuery,
    TeacherCourseDashboardListUseCase,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::infra::postgres::learning::teacher_course_dashboard_list_store::PostgresTeacherCourseDashboardListStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresTeacherCourseDashboardListUseCase {
    pool: DbPool,
}

impl PostgresTeacherCourseDashboardListUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherCourseDashboardListUseCase for PostgresTeacherCourseDashboardListUseCase {
    fn list_teacher_course_dashboard(
        &self,
        query: TeacherCourseDashboardListQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseDashboardListOutput, TeacherCourseDashboardError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherCourseDashboardListStore::new(&mut conn);
            list_teacher_course_dashboard::list_teacher_course_dashboard(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresTeacherCourseDashboardListUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TeacherCourseDashboardError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherCourseDashboardError::Connection(error.to_string()))
    }
}
