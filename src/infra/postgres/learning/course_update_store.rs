use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::update_course::{
    CourseUpdateError, CourseUpdateOutput, CourseUpdatePatch, CourseUpdateStore,
};
use crate::db::schema::courses;
use crate::infra::postgres::learning::course_permission_checks;
use crate::models::course::Course;

pub struct PostgresCourseUpdateStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseUpdateStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseUpdateStore for PostgresCourseUpdateStore<'_> {
    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseUpdateError>> {
        let permission = permission.to_string();
        async move {
            course_permission_checks::has_course_permission(
                self.conn,
                actor_user_id,
                course_id,
                &permission,
            )
            .await
            .map_err(map_course_update_error)
        }
        .boxed()
    }

    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseUpdateError>> {
        let permission = permission.to_string();
        async move {
            course_permission_checks::has_platform_permission(self.conn, actor_user_id, &permission)
                .await
                .map_err(map_course_update_error)
        }
        .boxed()
    }

    fn update_course(
        &mut self,
        course_id: i32,
        patch: CourseUpdatePatch,
    ) -> BoxFuture<'_, Result<CourseUpdateOutput, CourseUpdateError>> {
        async move {
            diesel::update(courses::table.find(course_id))
                .set(CourseUpdateChangeset::from(patch))
                .get_result::<Course>(self.conn)
                .await
                .map(course_update_output_from_model)
                .map_err(map_course_update_error)
        }
        .boxed()
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = courses)]
struct CourseUpdateChangeset {
    title: Option<String>,
    description: Option<Option<String>>,
    topics: Option<Option<String>>,
    prerequisites: Option<Option<String>>,
}

impl From<CourseUpdatePatch> for CourseUpdateChangeset {
    fn from(patch: CourseUpdatePatch) -> Self {
        Self {
            title: patch.title,
            description: patch.description,
            topics: patch.topics,
            prerequisites: patch.prerequisites,
        }
    }
}

fn map_course_update_error(error: diesel::result::Error) -> CourseUpdateError {
    match error {
        diesel::result::Error::NotFound => CourseUpdateError::NotFound,
        other => CourseUpdateError::Database(other.to_string()),
    }
}

fn course_update_output_from_model(course: Course) -> CourseUpdateOutput {
    CourseUpdateOutput {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        description: course.description,
        topics: course.topics,
        prerequisites: course.prerequisites,
    }
}
