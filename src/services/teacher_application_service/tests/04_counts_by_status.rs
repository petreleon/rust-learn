#[test]
fn counts_by_status() {
    let apps = vec![
        app_with_status(TEACHER_APPLICATION_STATUS_SUBMITTED),
        app_with_status(TEACHER_APPLICATION_STATUS_SUBMITTED),
        app_with_status(TEACHER_APPLICATION_STATUS_APPROVED),
        app_with_status(TEACHER_APPLICATION_STATUS_REJECTED),
        app_with_status(TEACHER_APPLICATION_STATUS_NEEDS_CHANGES),
    ];
    let summary = teacher_application_summary(&apps);
    assert_eq!(summary.total, 5);
    assert_eq!(summary.submitted, 2);
    assert_eq!(summary.approved, 1);
    assert_eq!(summary.rejected, 1);
    assert_eq!(summary.needs_changes, 1);
}

#[test]
fn empty_applications_produces_zeroes() {
    let summary = teacher_application_summary(&[]);
    assert_eq!(summary.total, 0);
    assert_eq!(summary.submitted, 0);
    assert_eq!(summary.approved, 0);
}

// ── build_audit_summaries ──
fn audit_event(application_id: i64, event_type: &str) -> TeacherApplicationAuditEvent {
    TeacherApplicationAuditEvent {
        id: application_id,
        application_id,
        actor_user_id: None,
        event_type: event_type.into(),
        from_status: None,
        to_status: "submitted".into(),
        reason: None,
        created_at: Utc::now(),
    }
}

#[test]
fn builds_summaries_from_events() {
    let events = vec![
        audit_event(1, "submitted"),
        audit_event(1, "reviewed"),
        audit_event(2, "submitted"),
    ];
    let summaries = build_audit_summaries(events);
    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[&1].event_count, 2);
    assert_eq!(summaries[&2].event_count, 1);
}

#[test]
fn latest_event_overwrites_previous() {
    let earlier = Utc::now();
    let later = earlier + chrono::Duration::hours(1);
    let events = vec![
        TeacherApplicationAuditEvent {
            id: 1,
            application_id: 1,
            actor_user_id: None,
            event_type: "first".into(),
            from_status: None,
            to_status: "".into(),
            reason: None,
            created_at: earlier,
        },
        TeacherApplicationAuditEvent {
            id: 2,
            application_id: 1,
            actor_user_id: None,
            event_type: "second".into(),
            from_status: None,
            to_status: "".into(),
            reason: Some("final reason".into()),
            created_at: later,
        },
    ];
    let summaries = build_audit_summaries(events);
    assert_eq!(summaries[&1].event_count, 2);
    assert_eq!(summaries[&1].latest_event_type, Some("second".into()));
}
