use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleError, CourseLifecycleOutput, CourseLifecycleStore,
};
use crate::db::schema::courses;
use crate::infra::postgres::learning::course_lifecycle_permissions;
use crate::models::course::Course;

pub struct PostgresCourseLifecycleStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseLifecycleStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseLifecycleStore for PostgresCourseLifecycleStore<'_> {
    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseLifecycleError>> {
        let permission = permission.to_string();
        async move {
            course_lifecycle_permissions::has_course_permission(
                self.conn,
                actor_user_id,
                course_id,
                &permission,
            )
            .await
            .map_err(map_lifecycle_error)
        }
        .boxed()
    }

    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseLifecycleError>> {
        let permission = permission.to_string();
        async move {
            course_lifecycle_permissions::has_platform_permission(
                self.conn,
                actor_user_id,
                &permission,
            )
            .await
            .map_err(map_lifecycle_error)
        }
        .boxed()
    }

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
