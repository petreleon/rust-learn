use std::collections::BTreeMap;

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewAuditSummaryOutput,
    TeacherApplicationPlatformReviewSummaryOutput,
};
use crate::domain::teacher_applications::status::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::models::teacher_application::{TeacherApplication, TeacherApplicationAuditEvent};

pub fn audit_summaries(
    audit_events: Vec<TeacherApplicationAuditEvent>,
) -> BTreeMap<i64, TeacherApplicationPlatformReviewAuditSummaryOutput> {
    let mut summaries = BTreeMap::<i64, TeacherApplicationPlatformReviewAuditSummaryOutput>::new();
    for event in audit_events {
        let summary = summaries.entry(event.application_id).or_default();
        summary.event_count += 1;
        summary.latest_event_type = Some(event.event_type);
        summary.latest_event_at = Some(event.created_at);
        summary.latest_reason = event.reason;
    }
    summaries
}

pub fn application_summary(
    applications: &[TeacherApplication],
) -> TeacherApplicationPlatformReviewSummaryOutput {
    let mut summary = TeacherApplicationPlatformReviewSummaryOutput::default();
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
