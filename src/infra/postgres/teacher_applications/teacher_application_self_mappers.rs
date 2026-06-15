use crate::application::teacher_applications::get_my_application::TeacherApplicationSelfError;
use crate::application::teacher_applications::TeacherApplicationOutput;
use crate::infra::postgres::models::teacher_application::TeacherApplication;

impl From<TeacherApplication> for TeacherApplicationOutput {
    fn from(application: TeacherApplication) -> Self {
        Self {
            applicant_user_id: application.applicant_user_id,
            created_at: application.created_at,
            decided_at: application.decided_at,
            decision_reason: application.decision_reason,
            experience_summary: application.experience_summary,
            id: application.id,
            idempotency_key: application.idempotency_key,
            organization_sponsor_id: application.organization_sponsor_id,
            portfolio_links: application.portfolio_links,
            requested_course_id: application.requested_course_id,
            requested_organization_id: application.requested_organization_id,
            requested_scope: application.requested_scope,
            reviewer_id: application.reviewer_id,
            status: application.status,
            updated_at: application.updated_at,
        }
    }
}

pub(super) fn map_error(error: diesel::result::Error) -> TeacherApplicationSelfError {
    TeacherApplicationSelfError::Database(error.to_string())
}
