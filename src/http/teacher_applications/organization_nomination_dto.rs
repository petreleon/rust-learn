use serde::{Deserialize, Serialize};

use crate::application::teacher_applications::nominate_application::TeacherApplicationNominationCommand;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct OrganizationTeacherNominationRequest {
    pub(super) applicant_user_id: i32,
    pub(super) requested_scope: Option<String>,
    pub(super) requested_course_id: Option<i32>,
    pub(super) experience_summary: String,
    pub(super) portfolio_links: Option<Vec<String>>,
    pub(super) idempotency_key: Option<String>,
}

impl OrganizationTeacherNominationRequest {
    pub(crate) fn into_command(
        self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> TeacherApplicationNominationCommand {
        TeacherApplicationNominationCommand {
            actor_user_id,
            applicant_user_id: self.applicant_user_id,
            experience_summary: self.experience_summary,
            idempotency_key: self.idempotency_key,
            organization_id,
            portfolio_links: self.portfolio_links,
            requested_course_id: self.requested_course_id,
            requested_scope: self.requested_scope,
        }
    }
}
