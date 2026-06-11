#[test]
fn builds_new_application() {
    let app = build_new_application(42, valid_request(), None).unwrap();
    assert_eq!(app.applicant_user_id, 42);
    assert_eq!(app.requested_scope, "platform");
    assert_eq!(app.status, TEACHER_APPLICATION_STATUS_SUBMITTED);
    assert_eq!(app.idempotency_key, Some("my-key".into()));
}

#[test]
fn rejects_empty_experience_summary() {
    let mut req = valid_request();
    req.experience_summary = "   ".into();
    assert!(build_new_application(42, req, None).is_err());
}

#[test]
fn forces_sponsor_id() {
    let app = build_new_application(42, valid_request(), Some(99)).unwrap();
    assert_eq!(app.organization_sponsor_id, Some(99));
}

#[test]
fn sponsored_org_scope_allows_missing_organization_id() {
    let mut req = valid_request();
    req.requested_scope = "organization".into();
    req.requested_organization_id = None;
    let app = build_new_application(42, req, Some(10)).unwrap();
    assert_eq!(app.requested_scope, "organization");
    assert_eq!(app.organization_sponsor_id, Some(10));
}

// ── ensure_idempotent_application_matches ──

fn existing_app() -> TeacherApplication {
    TeacherApplication {
        id: 1,
        applicant_user_id: 42,
        requested_scope: "platform".into(),
        requested_organization_id: None,
        requested_course_id: None,
        experience_summary: "I have taught for 5 years".into(),
        organization_sponsor_id: None,
        portfolio_links: json!(["https://port.link"]),
        status: TEACHER_APPLICATION_STATUS_SUBMITTED.into(),
        reviewer_id: None,
        decision_reason: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        decided_at: None,
        idempotency_key: Some("my-key".into()),
    }
}

fn matching_new_app() -> NewTeacherApplication {
    NewTeacherApplication {
        applicant_user_id: 42,
        requested_scope: "platform".into(),
        requested_organization_id: None,
        requested_course_id: None,
        experience_summary: "I have taught for 5 years".into(),
        organization_sponsor_id: None,
        portfolio_links: json!(["https://port.link"]),
        status: TEACHER_APPLICATION_STATUS_SUBMITTED.into(),
        idempotency_key: Some("my-key".into()),
    }
}

#[test]
fn matches_identical_application() {
    ensure_idempotent_application_matches(&existing_app(), &matching_new_app()).unwrap();
}

#[test]
fn mismatch_different_user_id() {
    let mut new_app = matching_new_app();
    new_app.applicant_user_id = 99;
    assert!(ensure_idempotent_application_matches(&existing_app(), &new_app).is_err());
}

#[test]
fn mismatch_different_scope() {
    let mut new_app = matching_new_app();
    new_app.requested_scope = "course".into();
    assert!(ensure_idempotent_application_matches(&existing_app(), &new_app).is_err());
}

#[test]
fn mismatch_different_experience() {
    let mut new_app = matching_new_app();
    new_app.experience_summary = "Different".into();
    assert!(ensure_idempotent_application_matches(&existing_app(), &new_app).is_err());
}

// ── teacher_application_summary ──

fn app_with_status(status: &str) -> TeacherApplication {
    let mut app = existing_app();
    app.status = status.into();
    app
}
