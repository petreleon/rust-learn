use crate::db::schema::{organization_roles, user_role_organization, users};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::collections::{BTreeMap, BTreeSet};

use super::dashboard_types_and_reads::{
    OrganizationDashboardAlert, OrganizationDashboardOperatorPermissions,
    OrganizationDashboardRewardSummary, OrganizationDashboardWalletSummary,
    OrganizationMemberBuilder,
};
use super::support::{
    OrganizationDashboardCourseSummary, OrganizationDashboardTeacherApplicationSummary,
};

pub(super) fn organization_dashboard_alerts(
    organization_id: i32,
    courses: &OrganizationDashboardCourseSummary,
    teacher_applications: &OrganizationDashboardTeacherApplicationSummary,
    rewards: &OrganizationDashboardRewardSummary,
    wallet: &OrganizationDashboardWalletSummary,
    permissions: &OrganizationDashboardOperatorPermissions,
) -> Vec<OrganizationDashboardAlert> {
    let mut alerts = Vec::new();
    if teacher_applications.available && teacher_applications.submitted > 0 {
        alerts.push(OrganizationDashboardAlert {
            severity: "warning".to_string(),
            kind: "teacher_applications_submitted".to_string(),
            message: format!(
                "{} sponsored teacher {} awaiting platform review.",
                teacher_applications.submitted,
                if teacher_applications.submitted == 1 {
                    "application is"
                } else {
                    "applications are"
                }
            ),
            action_label: Some("Open teacher nominations".to_string()),
            action_href: Some(format!(
                "/organizations/{organization_id}/teacher-applications"
            )),
        });
    }

    if courses.available && courses.needs_changes > 0 {
        alerts.push(OrganizationDashboardAlert {
            severity: "warning".to_string(),
            kind: "courses_need_changes".to_string(),
            message: format!(
                "{} sponsored course {} marked needs changes.",
                courses.needs_changes,
                if courses.needs_changes == 1 {
                    "is"
                } else {
                    "are"
                }
            ),
            action_label: Some("Open courses".to_string()),
            action_href: Some(format!("/organizations/{organization_id}/courses")),
        });
    }

    if rewards.available && rewards.failed_count + rewards.needs_reconciliation_count > 0 {
        let reward_attention_count = rewards.failed_count + rewards.needs_reconciliation_count;
        alerts.push(OrganizationDashboardAlert {
            severity: "warning".to_string(),
            kind: "reward_reconciliation".to_string(),
            message: if reward_attention_count == 1 {
                "1 reward record has failed or needs reconciliation.".to_string()
            } else {
                format!(
                    "{reward_attention_count} reward records have failed or need reconciliation."
                )
            },
            action_label: Some("Open reports".to_string()),
            action_href: Some(format!("/organizations/{organization_id}/reports")),
        });
    }

    if wallet.available
        && wallet.wallet_count == 0
        && (permissions.can_manage_wallets || permissions.can_manage_reward_budget)
    {
        alerts.push(OrganizationDashboardAlert {
            severity: "warning".to_string(),
            kind: "wallet_missing".to_string(),
            message: "No organization wallet is configured for reward budget operations."
                .to_string(),
            action_label: None,
            action_href: None,
        });
    }

    if alerts.is_empty() {
        alerts.push(OrganizationDashboardAlert {
            severity: "info".to_string(),
            kind: "no_attention_items".to_string(),
            message: "No dashboard attention items are visible for this session.".to_string(),
            action_label: None,
            action_href: None,
        });
    }

    alerts
}

pub(super) async fn build_organization_member_builders(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<BTreeMap<i32, OrganizationMemberBuilder>> {
    let rows =
        user_role_organization::table
            .inner_join(users::table.on(user_role_organization::user_id.eq(users::id.nullable())))
            .inner_join(organization_roles::table.on(
                user_role_organization::organization_role_id.eq(organization_roles::id.nullable()),
            ))
            .filter(user_role_organization::organization_id.eq(organization_id))
            .select((
                users::id,
                users::name,
                users::email,
                users::email_verified,
                users::kyc_verified,
                users::created_at,
                organization_roles::name,
            ))
            .load::<(
                i32,
                String,
                String,
                bool,
                bool,
                chrono::NaiveDateTime,
                String,
            )>(conn)
            .await?;

    let mut members = BTreeMap::<i32, OrganizationMemberBuilder>::new();
    for (user_id, name, email, email_verified, kyc_verified, created_at, role_name) in rows {
        members
            .entry(user_id)
            .or_insert_with(|| OrganizationMemberBuilder {
                id: user_id,
                name,
                email,
                email_verified,
                kyc_verified,
                joined_at: created_at,
                roles: BTreeSet::new(),
                direct_permissions: BTreeSet::new(),
                delegated_permissions: BTreeSet::new(),
            })
            .roles
            .insert(role_name);
    }

    Ok(members)
}
