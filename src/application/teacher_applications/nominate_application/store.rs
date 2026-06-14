use futures::future::BoxFuture;

use crate::application::teacher_applications::{
    submit_application::TeacherApplicationSubmission, TeacherApplicationOutput,
};

use super::TeacherApplicationNominationError;

pub trait TeacherApplicationNominationStore {
    fn has_organization_permission(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
        permission: String,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationNominationError>>;

    fn find_application_by_idempotency_key(
        &mut self,
        idempotency_key: String,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationNominationError>>;

    fn find_latest_application_for_applicant(
        &mut self,
        applicant_user_id: i32,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationNominationError>>;

    fn create_organization_nomination(
        &mut self,
        actor_user_id: i32,
        submission: TeacherApplicationSubmission,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationNominationError>>;
}
