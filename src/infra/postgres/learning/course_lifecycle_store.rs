use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleError, CourseLifecycleOutput, CourseLifecycleStore,
};
use crate::db::schema::courses;
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::models::course::Course;

pub struct PostgresCourseLifecycleStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseLifecycleStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseLifecycleStore for PostgresCourseLifecycleStore<'_> {
    fn update_status(
        &mut self,
        course_id: i32,
        status: String,
    ) -> BoxFuture<'_, Result<CourseLifecycleOutput, CourseLifecycleError>> {
        async move {
            diesel::update(courses::table.find(course_id))
                .set(courses::lifecycle_status.eq(status))
                .get_result::<Course>(self.conn)
                .await
                .map(course_lifecycle_output_from_model)
                .map_err(map_lifecycle_error)
        }
        .boxed()
    }
}

impl AccessDecisionStore for PostgresCourseLifecycleStore<'_> {
    type Error = CourseLifecycleError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, CourseLifecycleError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_lifecycle_error)
        }
        .boxed()
    }
}

fn map_lifecycle_error(error: diesel::result::Error) -> CourseLifecycleError {
    match error {
        diesel::result::Error::NotFound => CourseLifecycleError::NotFound,
        other => CourseLifecycleError::Database(other.to_string()),
    }
}

fn course_lifecycle_output_from_model(course: Course) -> CourseLifecycleOutput {
    CourseLifecycleOutput {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        description: course.description,
        topics: course.topics,
        prerequisites: course.prerequisites,
    }
}
