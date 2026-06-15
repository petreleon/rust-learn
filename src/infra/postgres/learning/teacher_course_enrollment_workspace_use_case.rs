use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_teacher_course_enrollment_workspace::{
    self, TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseEnrollmentWorkspaceQuery,
    TeacherCourseEnrollmentWorkspaceUseCase,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::infra::postgres::learning::teacher_course_enrollment_workspace_store::PostgresTeacherCourseEnrollmentWorkspaceStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresTeacherCourseEnrollmentWorkspaceUseCase {
    pool: DbPool,
}

impl PostgresTeacherCourseEnrollmentWorkspaceUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherCourseEnrollmentWorkspaceUseCase for PostgresTeacherCourseEnrollmentWorkspaceUseCase {
    fn get_teacher_course_enrollment_workspace(
        &self,
        query: TeacherCourseEnrollmentWorkspaceQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseDashboardError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherCourseEnrollmentWorkspaceStore::new(&mut conn);
            get_teacher_course_enrollment_workspace::get_teacher_course_enrollment_workspace(
                &mut store, query,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresTeacherCourseEnrollmentWorkspaceUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TeacherCourseDashboardError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherCourseDashboardError::Connection(error.to_string()))
    }
}
