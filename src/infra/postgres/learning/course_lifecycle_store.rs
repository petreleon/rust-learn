use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleError, CourseLifecycleOutput, CourseLifecycleStore,
};
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::schema::courses;

pub struct PostgresCourseLifecycleStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseLifecycleStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseLifecycleStore for PostgresCourseLifecycleStore<'_> {
    fn lifecycle_status(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<String, CourseLifecycleError>> {
        async move {
            courses::table
                .find(course_id)
                .select(courses::lifecycle_status)
                .first::<String>(self.conn)
                .await
                .map_err(map_lifecycle_error)
        }
        .boxed()
    }

    fn update_status(
        &mut self,
        course_id: i32,
        current_status: String,
        status: String,
    ) -> BoxFuture<'_, Result<CourseLifecycleOutput, CourseLifecycleError>> {
        async move {
            let target = courses::table
                .filter(courses::id.eq(course_id))
                .filter(courses::lifecycle_status.eq(current_status));

            match diesel::update(target)
                .set(courses::lifecycle_status.eq(status))
                .get_result::<Course>(self.conn)
                .await
            {
                Ok(course) => Ok(course_lifecycle_output_from_model(course)),
                Err(error) => Err(map_update_error(self.conn, course_id, error).await),
            }
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

async fn map_update_error(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    error: diesel::result::Error,
) -> CourseLifecycleError {
    match error {
        diesel::result::Error::NotFound => {
            let exists = course_exists(conn, course_id).await;
            if matches!(exists, Ok(true)) {
                CourseLifecycleError::StaleUpdate(
                    "course lifecycle changed; refresh and retry".to_string(),
                )
            } else {
                exists.err().unwrap_or(CourseLifecycleError::NotFound)
            }
        }
        other => CourseLifecycleError::Database(other.to_string()),
    }
}

async fn course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<bool, CourseLifecycleError> {
    courses::table
        .find(course_id)
        .select(courses::id)
        .first::<i32>(conn)
        .await
        .optional()
        .map(|id| id.is_some())
        .map_err(map_lifecycle_error)
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
