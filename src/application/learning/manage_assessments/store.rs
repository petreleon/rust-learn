use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::learning::manage_assessments::{
    AssessmentAuthoringError, AssessmentDraft, AuthoredAssessmentOutput,
};

pub trait AssessmentAuthoringStore: AccessDecisionStore<Error = AssessmentAuthoringError> {
    fn list_for_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AuthoredAssessmentOutput>, AssessmentAuthoringError>>;

    fn create_assessment(
        &mut self,
        course_id: i32,
        draft: AssessmentDraft,
    ) -> BoxFuture<'_, Result<AuthoredAssessmentOutput, AssessmentAuthoringError>>;

    fn update_assessment(
        &mut self,
        course_id: i32,
        assessment_id: i32,
        draft: AssessmentDraft,
    ) -> BoxFuture<'_, Result<AuthoredAssessmentOutput, AssessmentAuthoringError>>;
}
