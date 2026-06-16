use futures::future::BoxFuture;

use crate::application::learning::manage_assessments::{
    AssessmentAuthoringCommand, AssessmentAuthoringError, AssessmentUpdateCommand,
    AuthoredAssessmentOutput,
};

pub trait AssessmentAuthoringUseCase: Send + Sync {
    fn list_course_assessments_for_authoring(
        &self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AuthoredAssessmentOutput>, AssessmentAuthoringError>>;

    fn create_assessment(
        &self,
        command: AssessmentAuthoringCommand,
    ) -> BoxFuture<'_, Result<AuthoredAssessmentOutput, AssessmentAuthoringError>>;

    fn update_assessment(
        &self,
        command: AssessmentUpdateCommand,
    ) -> BoxFuture<'_, Result<AuthoredAssessmentOutput, AssessmentAuthoringError>>;
}
