use crate::application::learning::assessment::{AssessmentAttemptOutput, AssessmentReadError};
use crate::application::learning::ports::AssessmentReadStore;

pub async fn list_user_assessment_attempts(
    store: &mut impl AssessmentReadStore,
    assessment_id: i32,
    user_id: i32,
) -> Result<Vec<AssessmentAttemptOutput>, AssessmentReadError> {
    store.list_attempts_for_user(assessment_id, user_id).await
}
