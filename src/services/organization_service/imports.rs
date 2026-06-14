use crate::application::reporting::organization_reward_dashboard::{
    load_organization_reward_dashboard, OrganizationRewardDashboardError,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, courses_organizations, delegated_permissions, organization_roles, organizations,
    reward_candidates, role_permission_organization, teacher_applications, user_role_organization,
    users, wallets,
};
use crate::db::DbPool;
use crate::domain::rewards::candidate::status::{
    REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION,
};
use crate::domain::learning::course::status::{
    COURSE_STATUS_APPROVED, COURSE_STATUS_ARCHIVED, COURSE_STATUS_DRAFT,
    COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED, COURSE_STATUS_SUBMITTED,
    COURSE_STATUS_SUSPENDED,
};
use crate::infra::postgres::reporting::organization_reward_dashboard_store::PostgresOrganizationRewardDashboardStore;
use crate::models::course::Course;
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::organization::{NewOrganization, Organization, UpdateOrganization};
use crate::models::teacher_application::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::repositories::organization_repository::assign_role_to_user_in_organization;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

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
