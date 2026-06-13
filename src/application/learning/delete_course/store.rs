use futures::future::BoxFuture;

use crate::application::learning::delete_course::{CourseDeletionError, CourseDeletionOutcome};

pub trait CourseDeletionStore {
    fn delete(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<CourseDeletionOutcome, CourseDeletionError>>;
}
