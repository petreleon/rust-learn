use std::collections::{BTreeMap, BTreeSet};

use chrono::Utc;

use super::*;
use crate::infra::postgres::models::delegated_permission::DelegatedPermission;

#[test]
fn organization_builder_creates_new_entry() {
    let mut organizations = BTreeMap::new();
    let entry = organization_builder(&mut organizations, 1, "My Org".into());

    assert_eq!(entry.id, 1);
    assert_eq!(entry.name, "My Org");
    assert_eq!(organizations.len(), 1);
}

#[test]
fn organization_builder_returns_existing() {
    let mut organizations = BTreeMap::new();
    organization_builder(&mut organizations, 1, "First".into());
    organization_builder(&mut organizations, 1, "Ignored".into());

    assert_eq!(organizations[&1].name, "First");
    assert_eq!(organizations.len(), 1);
}

#[test]
fn course_builder_creates_new_entry() {
    let mut courses = BTreeMap::new();
    let entry = course_builder(&mut courses, 1, "Course 1".into(), "draft".into());

    assert_eq!(entry.id, 1);
    assert_eq!(entry.title, "Course 1");
    assert_eq!(entry.lifecycle_status, "draft");
    assert_eq!(courses.len(), 1);
}

#[test]
fn course_builder_returns_existing() {
    let mut courses = BTreeMap::new();
    course_builder(&mut courses, 1, "Original".into(), "draft".into());
    course_builder(&mut courses, 1, "Ignored".into(), "published".into());

    assert_eq!(courses[&1].title, "Original");
    assert_eq!(courses.len(), 1);
}

#[test]
fn sorted_vec_sorts_and_collects() {
    let mut set = BTreeSet::new();
    set.insert("c".into());
    set.insert("a".into());
    set.insert("b".into());

    assert_eq!(sorted_vec(set), vec!["a", "b", "c"]);
}

#[test]
fn effective_permissions_unions_direct_and_delegated() {
    let mut direct = BTreeSet::new();
    direct.insert("A".into());
    let mut delegated = BTreeSet::new();
    delegated.insert("B".into());
    delegated.insert("A".into());

    let result = effective_permissions(&direct, &delegated);

    assert_eq!(result.len(), 2);
    assert!(result.contains(&"A".to_string()));
    assert!(result.contains(&"B".to_string()));
}

#[test]
fn effective_permissions_accepts_empty_sets() {
    let result = effective_permissions(&BTreeSet::new(), &BTreeSet::new());

    assert!(result.is_empty());
}

#[test]
fn delegated_permission_session_resolves_org_and_course_labels() {
    let delegation = test_delegation(Some(1), Some(10));
    let org_labels = BTreeMap::from([(1, "ACME Corp".to_string())]);
    let course_labels = BTreeMap::from([(10, ("Rust 101".to_string(), "published".to_string()))]);

    let session = delegated_permission_session(delegation, &org_labels, &course_labels);

    assert_eq!(session.organization_name, Some("ACME Corp".to_string()));
    assert_eq!(session.course_title, Some("Rust 101".to_string()));
    assert_eq!(
        session.course_lifecycle_status,
        Some("published".to_string())
    );
}

#[test]
fn delegated_permission_session_leaves_missing_labels_empty() {
    let delegation = test_delegation(Some(999), Some(888));
    let org_labels = BTreeMap::new();
    let course_labels = BTreeMap::new();

    let session = delegated_permission_session(delegation, &org_labels, &course_labels);

    assert_eq!(session.organization_name, None);
    assert_eq!(session.course_title, None);
    assert_eq!(session.course_lifecycle_status, None);
}

fn test_delegation(org_id: Option<i32>, course_id: Option<i32>) -> DelegatedPermission {
    DelegatedPermission {
        id: 1,
        grantor_user_id: 10,
        grantee_user_id: 20,
        permission: "VIEW_COURSE_REWARD_STATUS".into(),
        scope_type: "course".into(),
        organization_id: org_id,
        course_id,
        reason: None,
        expires_at: None,
        created_at: Utc::now(),
        revoked_at: None,
        revoked_by_user_id: None,
        revoke_reason: None,
        updated_at: Utc::now(),
    }
}
