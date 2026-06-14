use crate::db::schema::organizations;
use crate::db::DbPool;
use crate::models::organization::Organization;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use serde::Serialize;
use std::collections::BTreeSet;

use super::support::{OrganizationMemberListError, OrganizationMemberListQuery};
use super::user_has_organization_dashboard_access::normalize_query_value;

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardRewardSummary {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub reward_candidate_count: i64,
    pub approved_reward_count: i64,
    pub approved_amount_total: String,
    pub failed_count: i64,
    pub needs_reconciliation_count: i64,
}

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardWalletSummary {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub wallet_count: i64,
    pub balance_total: String,
}

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardOperatorPermissions {
    pub can_view_dashboard: bool,
    pub can_view_members: bool,
    pub can_view_courses: bool,
    pub can_view_reports: bool,
    pub can_view_teacher_applications: bool,
    pub can_nominate_teachers: bool,
    pub can_manage_wallets: bool,
    pub can_manage_reward_budget: bool,
}

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardAlert {
    pub severity: String,
    pub kind: String,
    pub message: String,
    pub action_label: Option<String>,
    pub action_href: Option<String>,
}

#[derive(Debug)]
pub enum OrganizationDashboardError {
    PermissionDenied,
    NotFound,
    Database(DieselError),
    Reporting(String),
}

impl From<DieselError> for OrganizationDashboardError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => OrganizationDashboardError::NotFound,
            other => OrganizationDashboardError::Database(other),
        }
    }
}

#[derive(Debug)]
pub(super) struct OrganizationMemberBuilder {
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) email: String,
    pub(super) email_verified: bool,
    pub(super) kyc_verified: bool,
    pub(super) joined_at: chrono::NaiveDateTime,
    pub(super) roles: BTreeSet<String>,
    pub(super) direct_permissions: BTreeSet<String>,
    pub(super) delegated_permissions: BTreeSet<String>,
}

impl OrganizationMemberListQuery {
    pub fn new(
        search: Option<String>,
        role: Option<String>,
        permission: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        Self {
            search: normalize_query_value(search),
            role: normalize_query_value(role),
            permission: normalize_query_value(permission),
            limit: limit.unwrap_or(25).clamp(1, 100),
            offset: offset.unwrap_or(0).max(0),
        }
    }
}

impl From<DieselError> for OrganizationMemberListError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => OrganizationMemberListError::NotFound,
            other => OrganizationMemberListError::Database(other),
        }
    }
}

pub async fn list_organizations(pool: &DbPool) -> Result<Vec<Organization>, String> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| "Failed to get DB connection")?;
    organizations::table
        .load::<Organization>(&mut conn)
        .await
        .map_err(|e| format!("DB error: {}", e))
}

pub async fn get_organization(
    pool: &DbPool,
    org_id: i32,
) -> Result<Organization, diesel::result::Error> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| diesel::result::Error::NotFound)?; // Simplified error mapping
    organizations::table
        .find(org_id)
        .first::<Organization>(&mut conn)
        .await
}
