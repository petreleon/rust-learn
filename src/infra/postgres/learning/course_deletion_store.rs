use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::delete_course::{
    CourseDeletionError, CourseDeletionOutcome, CourseDeletionStore,
};
use crate::db::schema::courses;

pub struct PostgresCourseDeletionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseDeletionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseDeletionStore for PostgresCourseDeletionStore<'_> {
    fn delete(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<CourseDeletionOutcome, CourseDeletionError>> {
        async move {
            diesel::delete(courses::table.find(course_id))
                .execute(self.conn)
                .await
                .map(deletion_outcome)
                .map_err(|error| CourseDeletionError::Database(error.to_string()))
        }
        .boxed()
    }
}

fn deletion_outcome(deleted_count: usize) -> CourseDeletionOutcome {
    if deleted_count > 0 {
        CourseDeletionOutcome::Deleted
    } else {
        CourseDeletionOutcome::NotFound
    }
}
