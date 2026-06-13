use crate::application::learning::delete_course::{
    CourseDeletionError, CourseDeletionOutcome, CourseDeletionStore,
};

pub async fn delete_course(
    store: &mut impl CourseDeletionStore,
    course_id: i32,
) -> Result<CourseDeletionOutcome, CourseDeletionError> {
    store.delete(course_id).await
}
