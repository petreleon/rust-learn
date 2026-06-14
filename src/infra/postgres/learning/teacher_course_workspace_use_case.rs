use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_teacher_course_workspace::{
    self, TeacherCourseWorkspaceOutput, TeacherCourseWorkspaceQuery, TeacherCourseWorkspaceUseCase,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::db::DbPool;
use crate::infra::postgres::learning::teacher_course_workspace_store::PostgresTeacherCourseWorkspaceStore;

#[derive(Clone)]
pub struct PostgresTeacherCourseWorkspaceUseCase {
    pool: DbPool,
}

impl PostgresTeacherCourseWorkspaceUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherCourseWorkspaceUseCase for PostgresTeacherCourseWorkspaceUseCase {
    fn get_teacher_course_workspace(
        &self,
        query: TeacherCourseWorkspaceQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseWorkspaceOutput, TeacherCourseDashboardError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherCourseWorkspaceStore::new(&mut conn);
            get_teacher_course_workspace::get_teacher_course_workspace(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresTeacherCourseWorkspaceUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TeacherCourseDashboardError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherCourseDashboardError::Connection(error.to_string()))
    }
}
