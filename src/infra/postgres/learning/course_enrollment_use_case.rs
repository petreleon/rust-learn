use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::course_enrollment::{
    self, CourseEnrollmentError, CourseEnrollmentRemovalOutput, CourseEnrollmentUseCase,
    CourseJoinDecisionOutput, CourseJoinRequestOutput, DecideCourseJoinCommand,
    RemoveCourseEnrollmentCommand, RequestCourseJoinCommand,
};
use crate::infra::postgres::learning::course_enrollment_store::PostgresCourseEnrollmentStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresCourseEnrollmentUseCase {
    pool: DbPool,
}

impl PostgresCourseEnrollmentUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseEnrollmentUseCase for PostgresCourseEnrollmentUseCase {
    fn request_course_join(
        &self,
        command: RequestCourseJoinCommand,
    ) -> BoxFuture<'_, Result<CourseJoinRequestOutput, CourseEnrollmentError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseEnrollmentStore::new(&mut conn);
            course_enrollment::request_course_join(&mut store, command).await
        }
        .boxed()
    }

    fn decide_course_join_request(
        &self,
        command: DecideCourseJoinCommand,
    ) -> BoxFuture<'_, Result<CourseJoinDecisionOutput, CourseEnrollmentError>> {
        async move {
            let mut conn = self.connection().await?;
            conn.transaction::<_, CourseEnrollmentError, _>(|conn| {
                Box::pin(async move {
                    let mut store = PostgresCourseEnrollmentStore::new(conn);
                    course_enrollment::decide_course_join_request(&mut store, command).await
                })
            })
            .await
        }
        .boxed()
    }

    fn remove_course_enrollment(
        &self,
        command: RemoveCourseEnrollmentCommand,
    ) -> BoxFuture<'_, Result<CourseEnrollmentRemovalOutput, CourseEnrollmentError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseEnrollmentStore::new(&mut conn);
            course_enrollment::remove_course_enrollment(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresCourseEnrollmentUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseEnrollmentError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseEnrollmentError::Connection(error.to_string()))
    }
}
