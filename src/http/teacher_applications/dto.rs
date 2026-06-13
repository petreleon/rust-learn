use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use crate::application::teacher_applications::get_my_application::{
    TeacherApplicationAuditEventOutput, TeacherApplicationOutput, TeacherApplicationSelfOutput,
};

#[derive(Debug, Serialize)]
pub(super) struct TeacherApplicationSelfResponse {
    pub(super) application: Option<TeacherApplicationResponse>,
    pub(super) audit_events: Vec<TeacherApplicationAuditEventResponse>,
}

#[derive(Debug, Serialize)]
pub(super) struct TeacherApplicationResponse {
    pub(super) id: i64,
    pub(super) applicant_user_id: i32,
    pub(super) requested_scope: String,
    pub(super) requested_organization_id: Option<i32>,
    pub(super) requested_course_id: Option<i32>,
    pub(super) experience_summary: String,
    pub(super) organization_sponsor_id: Option<i32>,
    pub(super) portfolio_links: Value,
    pub(super) status: String,
    pub(super) reviewer_id: Option<i32>,
    pub(super) decision_reason: Option<String>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
    pub(super) decided_at: Option<DateTime<Utc>>,
    pub(super) idempotency_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub(super) struct TeacherApplicationAuditEventResponse {
    pub(super) id: i64,
    pub(super) application_id: i64,
    pub(super) actor_user_id: Option<i32>,
    pub(super) event_type: String,
    pub(super) from_status: Option<String>,
    pub(super) to_status: String,
    pub(super) reason: Option<String>,
    pub(super) created_at: DateTime<Utc>,
}

impl From<TeacherApplicationSelfOutput> for TeacherApplicationSelfResponse {
    fn from(output: TeacherApplicationSelfOutput) -> Self {
        Self {
            application: output.application.map(Into::into),
            audit_events: output.audit_events.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<TeacherApplicationOutput> for TeacherApplicationResponse {
    fn from(application: TeacherApplicationOutput) -> Self {
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

impl From<TeacherApplicationAuditEventOutput> for TeacherApplicationAuditEventResponse {
    fn from(event: TeacherApplicationAuditEventOutput) -> Self {
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
