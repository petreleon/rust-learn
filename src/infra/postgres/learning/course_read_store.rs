use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_course::{CourseOutput, CourseReadError, CourseReadStore};
use crate::db::schema::courses;
use crate::infra::postgres::models::course::Course;

pub struct PostgresCourseReadStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseReadStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseReadStore for PostgresCourseReadStore<'_> {
    fn get(&mut self, course_id: i32) -> BoxFuture<'_, Result<CourseOutput, CourseReadError>> {
        async move {
            courses::table
                .find(course_id)
                .first::<Course>(self.conn)
                .await
                .map(course_output_from_model)
                .map_err(map_course_read_error)
        }
        .boxed()
    }
}

fn map_course_read_error(error: diesel::result::Error) -> CourseReadError {
    match error {
        diesel::result::Error::NotFound => CourseReadError::NotFound,
        other => CourseReadError::Database(other.to_string()),
    }
}

fn course_output_from_model(course: Course) -> CourseOutput {
    CourseOutput {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        description: course.description,
        topics: course.topics,
        prerequisites: course.prerequisites,
    }
}
