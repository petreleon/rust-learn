use crate::models::delegated_permission::DelegatedPermission;
use crate::models::user::User;
use crate::repositories::delegated_permission_repository::{self, DelegatedPermissionFilter};
use crate::repositories::session_repository;
use chrono::{DateTime, Utc};
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq)]
pub enum CurrentSessionError {
    MissingUser,
    EmailUnverified,
    Database(String),
}

impl From<DieselError> for CurrentSessionError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => CurrentSessionError::MissingUser,
            other => CurrentSessionError::Database(other.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CurrentSessionResponse {
    pub user: CurrentSessionUser,
    pub platform: PlatformSessionScope,
    pub organizations: Vec<OrganizationSessionScope>,
    pub courses: Vec<CourseSessionScope>,
    pub delegated_permissions: Vec<DelegatedPermissionSession>,
}

#[derive(Debug, Serialize)]
pub struct CurrentSessionUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct PlatformSessionScope {
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationSessionScope {
    pub id: i32,
    pub name: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CourseSessionScope {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DelegatedPermissionSession {
    pub id: i64,
    pub grantor_user_id: i32,
    pub permission: String,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub organization_name: Option<String>,
    pub course_id: Option<i32>,
    pub course_title: Option<String>,
    pub course_lifecycle_status: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default)]
struct PlatformScopeBuilder {
    roles: BTreeSet<String>,
    direct_permissions: BTreeSet<String>,
    delegated_permissions: BTreeSet<String>,
}

#[derive(Debug)]
struct OrganizationScopeBuilder {
    id: i32,
    name: String,
    roles: BTreeSet<String>,
    direct_permissions: BTreeSet<String>,
    delegated_permissions: BTreeSet<String>,
}

#[derive(Debug)]
struct CourseScopeBuilder {
    id: i32,
    title: String,
    lifecycle_status: String,
    roles: BTreeSet<String>,
    direct_permissions: BTreeSet<String>,
    delegated_permissions: BTreeSet<String>,
}
