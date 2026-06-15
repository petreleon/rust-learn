use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::assign_course_role::{
    self, CourseRoleAssignmentCommand, CourseRoleAssignmentError, CourseRoleAssignmentOutput,
    CourseRoleAssignmentUseCase,
};
use crate::infra::postgres::learning::course_role_assignment_store::PostgresCourseRoleAssignmentStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresCourseRoleAssignmentUseCase {
    pool: DbPool,
}

impl PostgresCourseRoleAssignmentUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseRoleAssignmentUseCase for PostgresCourseRoleAssignmentUseCase {
    fn assign_course_role(
        &self,
        command: CourseRoleAssignmentCommand,
    ) -> BoxFuture<'_, Result<CourseRoleAssignmentOutput, CourseRoleAssignmentError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseRoleAssignmentStore::new(&mut conn);
            assign_course_role::assign_course_role(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresCourseRoleAssignmentUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseRoleAssignmentError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseRoleAssignmentError::Connection(error.to_string()))
    }
}
