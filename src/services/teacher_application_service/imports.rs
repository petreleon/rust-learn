use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, organizations, teacher_application_audit_events, teacher_applications, users,
};
use crate::models::teacher_application::{
    NewTeacherApplication, NewTeacherApplicationAuditEvent, TeacherApplication,
    TeacherApplicationAuditEvent, TEACHER_APPLICATION_SCOPE_COURSE,
    TEACHER_APPLICATION_SCOPE_ORGANIZATION, TEACHER_APPLICATION_SCOPE_PLATFORM,
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::teacher_application_repository;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq)]
pub enum TeacherApplicationError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidTransition(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for TeacherApplicationError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => TeacherApplicationError::NotFound,
            other => TeacherApplicationError::Database(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitTeacherApplicationRequest {
    pub requested_scope: String,
    pub requested_organization_id: Option<i32>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub organization_sponsor_id: Option<i32>,
    pub portfolio_links: Option<Vec<String>>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrganizationTeacherNominationRequest {
    pub applicant_user_id: i32,
    pub requested_scope: Option<String>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub portfolio_links: Option<Vec<String>>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct OrganizationTeacherApplicationsRequest {
    pub status: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationTeacherApplicationsResponse {
    pub organization: OrganizationTeacherApplicationsOrganization,
    pub applications: Vec<OrganizationTeacherApplicationItem>,
    pub summary: TeacherApplicationDashboardSummary,
    pub operator_permissions: OrganizationTeacherApplicationPermissions,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationTeacherApplicationsOrganization {
    pub id: i32,
    pub name: String,
}
