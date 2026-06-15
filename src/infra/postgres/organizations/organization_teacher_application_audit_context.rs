use std::collections::BTreeMap;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationAuditSummaryOutput, OrganizationTeacherApplicationListError,
    TeacherApplicationDashboardSummaryOutput,
};
use crate::db::schema::teacher_application_audit_events;
use crate::domain::teacher_applications::audit::TeacherApplicationAuditEventType;
use crate::domain::teacher_applications::status::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::infra::postgres::models::teacher_application::{
    TeacherApplication, TeacherApplicationAuditEvent,
};

pub async fn load_audits(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<
    BTreeMap<i64, OrganizationTeacherApplicationAuditSummaryOutput>,
    OrganizationTeacherApplicationListError,
> {
    let application_ids = applications
        .iter()
        .map(|application| application.id)
        .collect::<Vec<_>>();
    if application_ids.is_empty() {
        return Ok(BTreeMap::new());
    }

    let audit_events = teacher_application_audit_events::table
        .filter(teacher_application_audit_events::application_id.eq_any(&application_ids))
        .order(teacher_application_audit_events::created_at.asc())
        .then_order_by(teacher_application_audit_events::id.asc())
        .load::<TeacherApplicationAuditEvent>(conn)
        .await
        .map_err(map_error)?;

    let mut summaries = BTreeMap::<i64, OrganizationTeacherApplicationAuditSummaryOutput>::new();
    for event in audit_events {
        let event_type =
            TeacherApplicationAuditEventType::parse(&event.event_type).map_err(|error| {
                OrganizationTeacherApplicationListError::Database(error.to_string())
            })?;
        let summary = summaries.entry(event.application_id).or_default();
        summary.event_count += 1;
        summary.latest_event_type = Some(event_type);
        summary.latest_event_at = Some(event.created_at);
        summary.latest_reason = event.reason;
    }
    Ok(summaries)
}

pub fn teacher_application_summary(
    applications: &[TeacherApplication],
) -> TeacherApplicationDashboardSummaryOutput {
    let mut summary = TeacherApplicationDashboardSummaryOutput::default();
    for application in applications {
        summary.total += 1;
        match application.status.as_str() {
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved += 1,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected += 1,
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted += 1,
            _ => {}
        }
    }
    summary
}

fn map_error(error: diesel::result::Error) -> OrganizationTeacherApplicationListError {
    OrganizationTeacherApplicationListError::Database(error.to_string())
}
