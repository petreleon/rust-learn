// ── organization_application_matches_search ──

fn org_item(
    name: &str,
    email: &str,
    status: &str,
    scope: &str,
) -> OrganizationTeacherApplicationItem {
    OrganizationTeacherApplicationItem {
        id: 1,
        applicant: TeacherApplicationUserSummary {
            id: 1,
            name: name.into(),
            email: email.into(),
        },
        requested_scope: scope.into(),
        requested_organization: None,
        requested_course: None,
        sponsored_by_this_organization: false,
        requested_for_this_organization: false,
        experience_summary: "".into(),
        portfolio_links: vec![],
        status: status.into(),
        reviewer: None,
        decision_reason: None,
        audit: TeacherApplicationAuditSummary::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        decided_at: None,
    }
}

#[test]
fn matches_by_applicant_name() {
    let item = org_item("Alice Smith", "a@e.com", "submitted", "course");
    assert!(organization_application_matches_search(&item, "alice"));
    assert!(!organization_application_matches_search(&item, "bob"));
}

#[test]
fn matches_by_status() {
    let item = org_item("A", "a@e.com", "submitted", "course");
    assert!(organization_application_matches_search(&item, "submitted"));
}

#[test]
fn matches_by_scope() {
    let item = org_item("A", "a@e.com", "s", "organization");
    assert!(organization_application_matches_search(
        &item,
        "organization"
    ));
}

// ── platform_application_matches_search ──

fn platform_item(id: i64, name: &str, email: &str, status: &str) -> PlatformTeacherApplicationItem {
    PlatformTeacherApplicationItem {
        id,
        applicant: TeacherApplicationUserSummary {
            id: 1,
            name: name.into(),
            email: email.into(),
        },
        requested_scope: "platform".into(),
        requested_organization: None,
        requested_course: None,
        sponsor_organization: None,
        experience_summary: "".into(),
        portfolio_links: vec![],
        status: status.into(),
        reviewer: None,
        decision_reason: None,
        audit: TeacherApplicationAuditSummary::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        decided_at: None,
    }
}

#[test]
fn matches_by_application_id() {
    let item = platform_item(42, "A", "a@e.com", "submitted");
    assert!(platform_application_matches_search(&item, "42"));
}

#[test]
fn matches_by_status_differs_from_org_search() {
    let item = platform_item(1, "Alice", "a@e.com", "approved");
    assert!(platform_application_matches_search(&item, "approved"));
}

#[test]
fn does_not_match_unrelated_text() {
    let item = platform_item(1, "Alice", "a@e.com", "submitted");
    assert!(!platform_application_matches_search(&item, "xyzzy"));
}
