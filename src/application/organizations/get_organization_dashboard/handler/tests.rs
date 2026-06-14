mod fake_store;
mod fake_store_summaries;

use futures::executor::block_on;

use super::get_organization_dashboard;
use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardError, OrganizationDashboardQuery,
};
use fake_store::FakeOrganizationDashboardStore;

#[test]
fn gates_sensitive_sections_without_loading_them() {
    let mut store = FakeOrganizationDashboardStore::limited();
    let dashboard = block_on(get_organization_dashboard(
        &mut store,
        OrganizationDashboardQuery {
            actor_user_id: 7,
            organization_id: 9,
        },
    ))
    .expect("dashboard should load for scoped member");

    assert!(dashboard.members.available);
    assert!(dashboard.courses.available);
    assert!(!dashboard.teacher_applications.available);
    assert!(!dashboard.rewards.available);
    assert!(!dashboard.wallet.available);
    assert_eq!(dashboard.health.status, "attention");
    assert!(dashboard
        .alerts
        .iter()
        .any(|alert| alert.kind == "courses_need_changes"));
    assert!(store.loaded_members);
    assert!(store.loaded_courses);
    assert!(!store.loaded_teacher_applications);
    assert!(!store.loaded_rewards);
    assert!(!store.loaded_wallet);
}

#[test]
fn builds_attention_alerts_from_loaded_sections() {
    let mut store = FakeOrganizationDashboardStore::full_access();
    let dashboard = block_on(get_organization_dashboard(
        &mut store,
        OrganizationDashboardQuery {
            actor_user_id: 7,
            organization_id: 9,
        },
    ))
    .expect("dashboard should load for admin");

    assert_eq!(dashboard.health.status, "attention");
    assert!(dashboard
        .alerts
        .iter()
        .any(|alert| alert.kind == "courses_need_changes"));
    assert!(dashboard
        .alerts
        .iter()
        .any(|alert| alert.kind == "reward_reconciliation"));
    assert!(dashboard
        .alerts
        .iter()
        .any(|alert| alert.kind == "wallet_missing"));
    assert!(store.loaded_teacher_applications);
    assert!(store.loaded_rewards);
    assert!(store.loaded_wallet);
}

#[test]
fn rejects_without_loading_permissions_or_summaries() {
    let mut store = FakeOrganizationDashboardStore::denied();
    let error = block_on(get_organization_dashboard(
        &mut store,
        OrganizationDashboardQuery {
            actor_user_id: 7,
            organization_id: 9,
        },
    ))
    .expect_err("dashboard should be permission gated");

    assert_eq!(error, OrganizationDashboardError::PermissionDenied);
    assert!(!store.loaded_permissions);
    assert!(!store.loaded_members);
}
