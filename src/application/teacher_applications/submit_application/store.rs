use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::teacher_applications::{
    submit_application::{TeacherApplicationSubmission, TeacherApplicationSubmitError},
    TeacherApplicationOutput,
};

pub trait TeacherApplicationSubmitStore:
    AccessDecisionStore<Error = TeacherApplicationSubmitError>
{
    fn find_application_by_idempotency_key(
        &mut self,
        idempotency_key: String,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationSubmitError>>;

    fn find_latest_application_for_applicant(
        &mut self,
        applicant_user_id: i32,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationSubmitError>>;

    fn create_submitted_application(
        &mut self,
        actor_user_id: i32,
        submission: TeacherApplicationSubmission,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationSubmitError>>;
}
