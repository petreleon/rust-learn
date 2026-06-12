use crate::application::learning::assessment::{AssessmentOutput, AssessmentReadError};
use crate::application::learning::ports::AssessmentReadStore;

pub async fn list_published_course_assessments(
    store: &mut impl AssessmentReadStore,
    course_id: i32,
) -> Result<Vec<AssessmentOutput>, AssessmentReadError> {
    store.list_published_for_course(course_id).await
}
