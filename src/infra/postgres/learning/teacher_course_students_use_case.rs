use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_teacher_course_students::{
    self, TeacherCourseStudentsOutput, TeacherCourseStudentsQuery, TeacherCourseStudentsUseCase,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::db::DbPool;
use crate::infra::postgres::learning::teacher_course_students_store::PostgresTeacherCourseStudentsStore;

#[derive(Clone)]
pub struct PostgresTeacherCourseStudentsUseCase {
    pool: DbPool,
}

impl PostgresTeacherCourseStudentsUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherCourseStudentsUseCase for PostgresTeacherCourseStudentsUseCase {
    fn get_teacher_course_students(
        &self,
        query: TeacherCourseStudentsQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseStudentsOutput, TeacherCourseDashboardError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherCourseStudentsStore::new(&mut conn);
            get_teacher_course_students::get_teacher_course_students(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresTeacherCourseStudentsUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TeacherCourseDashboardError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherCourseDashboardError::Connection(error.to_string()))
    }
}
