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
