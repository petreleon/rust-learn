use futures::future::BoxFuture;

use crate::application::learning::delete_course::{CourseDeletionError, CourseDeletionOutcome};

pub trait CourseDeletionUseCase: Send + Sync {
    fn delete_course(
        &self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<CourseDeletionOutcome, CourseDeletionError>>;
}
