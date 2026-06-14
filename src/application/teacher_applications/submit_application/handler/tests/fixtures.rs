use chrono::Utc;

use crate::application::teacher_applications::{
    submit_application::{TeacherApplicationSubmission, TeacherApplicationSubmitCommand},
    TeacherApplicationOutput,
};

pub(super) fn command() -> TeacherApplicationSubmitCommand {
    TeacherApplicationSubmitCommand {
        actor_user_id: 9,
        requested_scope: "platform".to_string(),
        experience_summary: "Experience".to_string(),
        idempotency_key: Some("retry".to_string()),
        ..Default::default()
    }
}

pub(super) fn application() -> TeacherApplicationOutput {
    let now = Utc::now();
    TeacherApplicationOutput {
        applicant_user_id: 9,
        created_at: now,
        decided_at: None,
        decision_reason: None,
        experience_summary: "Experience".to_string(),
        id: 99,
        idempotency_key: Some("retry".to_string()),
        organization_sponsor_id: None,
        portfolio_links: serde_json::json!([]),
        requested_course_id: None,
        requested_organization_id: None,
        requested_scope: "platform".to_string(),
        reviewer_id: None,
        status: "submitted".to_string(),
        updated_at: now,
    }
}

pub(super) fn application_from_submission(
    id: i64,
    submission: TeacherApplicationSubmission,
) -> TeacherApplicationOutput {
    let now = Utc::now();
    TeacherApplicationOutput {
        applicant_user_id: submission.applicant_user_id,
        created_at: now,
        decided_at: None,
        decision_reason: None,
        experience_summary: submission.experience_summary,
        id,
        idempotency_key: submission.idempotency_key,
        organization_sponsor_id: submission.organization_sponsor_id,
        portfolio_links: submission.portfolio_links,
        requested_course_id: submission.requested_course_id,
        requested_organization_id: submission.requested_organization_id,
        requested_scope: submission.requested_scope,
        reviewer_id: None,
        status: submission.status,
        updated_at: now,
    }
}
