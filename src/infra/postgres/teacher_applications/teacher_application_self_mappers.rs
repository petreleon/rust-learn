use crate::application::teacher_applications::get_my_application::{
    TeacherApplicationOutput, TeacherApplicationSelfError,
};
use crate::application::teacher_applications::TeacherApplicationAuditEventOutput;
use crate::models::teacher_application::{TeacherApplication, TeacherApplicationAuditEvent};

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

impl From<TeacherApplicationAuditEvent> for TeacherApplicationAuditEventOutput {
    fn from(event: TeacherApplicationAuditEvent) -> Self {
        Self {
            actor_user_id: event.actor_user_id,
            application_id: event.application_id,
            created_at: event.created_at,
            event_type: event.event_type,
            from_status: event.from_status,
            id: event.id,
            reason: event.reason,
            to_status: event.to_status,
        }
    }
}

pub(super) fn map_error(error: diesel::result::Error) -> TeacherApplicationSelfError {
    TeacherApplicationSelfError::Database(error.to_string())
}
