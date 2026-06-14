use diesel::result::Error as DieselError;
use serde::Serialize;

use super::dashboard_types_and_reads::{
    OrganizationDashboardAlert, OrganizationDashboardOperatorPermissions,
    OrganizationDashboardRewardSummary, OrganizationDashboardWalletSummary,
};

#[derive(Debug)]
pub struct OrganizationMemberListQuery {
    pub search: Option<String>,
    pub role: Option<String>,
    pub permission: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize)]
pub struct OrganizationMemberListResponse {
    pub organization: OrganizationMemberListOrganization,
    pub members: Vec<OrganizationMemberListItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub role: Option<String>,
    pub permission: Option<String>,
    pub operator_permissions: OrganizationMemberOperatorPermissions,
}

#[derive(Debug, Serialize)]
pub struct OrganizationMemberListOrganization {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct OrganizationMemberListItem {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
    pub joined_at: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
    pub direct_permission_count: usize,
    pub delegated_permission_count: usize,
    pub effective_permission_count: usize,
}

#[derive(Debug, Serialize)]
pub struct OrganizationMemberOperatorPermissions {
    pub can_view_members: bool,
    pub can_invite_members: bool,
    pub can_manage_members: bool,
    pub can_assign_roles: bool,
    pub can_manage_settings: bool,
}

#[derive(Debug)]
pub enum OrganizationMemberListError {
    PermissionDenied,
    NotFound,
    Database(DieselError),
}

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardResponse {
    pub organization: OrganizationDashboardOrganization,
    pub health: OrganizationDashboardHealth,
    pub members: OrganizationDashboardMemberSummary,
    pub courses: OrganizationDashboardCourseSummary,
    pub teacher_applications: OrganizationDashboardTeacherApplicationSummary,
    pub rewards: OrganizationDashboardRewardSummary,
    pub wallet: OrganizationDashboardWalletSummary,
    pub operator_permissions: OrganizationDashboardOperatorPermissions,
    pub alerts: Vec<OrganizationDashboardAlert>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardOrganization {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardHealth {
    pub status: String,
    pub alert_count: usize,
}

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardMemberSummary {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub total: i64,
    pub verified_email_count: i64,
    pub kyc_ready_count: i64,
    pub delegated_permission_count: i64,
}

#[derive(Debug, Serialize)]
pub struct OrganizationDashboardCourseSummary {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub total: i64,
    pub draft: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub published: i64,
    pub suspended: i64,
    pub archived: i64,
}

#[derive(Debug, Serialize, Default)]
pub struct OrganizationDashboardTeacherApplicationSummary {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}
