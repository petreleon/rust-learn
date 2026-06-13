fn build_audit_summaries(
    audit_events: Vec<TeacherApplicationAuditEvent>,
) -> BTreeMap<i64, TeacherApplicationAuditSummary> {
    let mut summaries = BTreeMap::<i64, TeacherApplicationAuditSummary>::new();
    for event in audit_events {
        let summary = summaries.entry(event.application_id).or_default();
        summary.event_count += 1;
        summary.latest_event_type = Some(event.event_type);
        summary.latest_event_at = Some(event.created_at);
        summary.latest_reason = event.reason;
    }
    summaries
}

fn teacher_application_summary(
    applications: &[TeacherApplication],
) -> TeacherApplicationDashboardSummary {
    let mut summary = TeacherApplicationDashboardSummary::default();
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

fn organization_application_matches_search(
    application: &OrganizationTeacherApplicationItem,
    search: &str,
) -> bool {
    application.applicant.name.to_lowercase().contains(search)
        || application.applicant.email.to_lowercase().contains(search)
        || application
            .experience_summary
            .to_lowercase()
            .contains(search)
        || application.status.to_lowercase().contains(search)
        || application.requested_scope.to_lowercase().contains(search)
        || application
            .requested_course
            .as_ref()
            .is_some_and(|course| course.title.to_lowercase().contains(search))
        || application
            .requested_organization
            .as_ref()
            .is_some_and(|organization| organization.name.to_lowercase().contains(search))
}
