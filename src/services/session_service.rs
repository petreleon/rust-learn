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

pub async fn current_session(
    conn: &mut AsyncPgConnection,
    current_user_id: i32,
) -> Result<CurrentSessionResponse, CurrentSessionError> {
    let user = session_repository::find_user(conn, current_user_id).await?;
    if !user.email_verified {
        return Err(CurrentSessionError::EmailUnverified);
    }

    let mut platform = PlatformScopeBuilder {
        roles: session_repository::list_platform_roles(conn, current_user_id)
            .await?
            .into_iter()
            .collect(),
        direct_permissions: session_repository::list_platform_permissions(conn, current_user_id)
            .await?
            .into_iter()
            .collect(),
        delegated_permissions: BTreeSet::new(),
    };

    let mut organizations = BTreeMap::<i32, OrganizationScopeBuilder>::new();
    for row in session_repository::list_organization_roles(conn, current_user_id).await? {
        organization_builder(
            &mut organizations,
            row.organization_id,
            row.organization_name,
        )
        .roles
        .insert(row.role_name);
    }

    for row in session_repository::list_organization_permissions(conn, current_user_id).await? {
        organization_builder(
            &mut organizations,
            row.organization_id,
            row.organization_name,
        )
        .direct_permissions
        .insert(row.permission);
    }

    let mut courses = BTreeMap::<i32, CourseScopeBuilder>::new();
    for row in session_repository::list_course_roles(conn, current_user_id).await? {
        course_builder(
            &mut courses,
            row.course_id,
            row.course_title,
            row.lifecycle_status,
        )
        .roles
        .insert(row.role_name);
    }

    for row in session_repository::list_course_permissions(conn, current_user_id).await? {
        course_builder(
            &mut courses,
            row.course_id,
            row.course_title,
            row.lifecycle_status,
        )
        .direct_permissions
        .insert(row.permission);
    }

    let delegations = delegated_permission_repository::list_delegated_permissions(
        conn,
        DelegatedPermissionFilter {
            grantee_user_id: Some(current_user_id),
            active: Some(true),
            limit: Some(500),
            ..DelegatedPermissionFilter::default()
        },
    )
    .await?;

    let organization_label_ids = delegations
        .iter()
        .filter_map(|delegation| delegation.organization_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let organization_labels =
        session_repository::organization_labels(conn, &organization_label_ids)
            .await?
            .into_iter()
            .collect::<BTreeMap<_, _>>();

    let course_label_ids = delegations
        .iter()
        .filter_map(|delegation| delegation.course_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let course_labels = session_repository::course_labels(conn, &course_label_ids)
        .await?
        .into_iter()
        .map(|(id, title, lifecycle_status)| (id, (title, lifecycle_status)))
        .collect::<BTreeMap<_, _>>();

    for delegation in &delegations {
        match delegation.scope_type.as_str() {
            "platform" => {
                platform
                    .delegated_permissions
                    .insert(delegation.permission.clone());
            }
            "organization" => {
                if let Some(organization_id) = delegation.organization_id {
                    let organization_name = organization_labels
                        .get(&organization_id)
                        .cloned()
                        .unwrap_or_else(|| format!("Organization {organization_id}"));
                    organization_builder(&mut organizations, organization_id, organization_name)
                        .delegated_permissions
                        .insert(delegation.permission.clone());
                }
            }
            "course" => {
                if let Some(course_id) = delegation.course_id {
                    let (course_title, lifecycle_status) = course_labels
                        .get(&course_id)
                        .cloned()
                        .unwrap_or_else(|| (format!("Course {course_id}"), "unknown".to_string()));
                    course_builder(&mut courses, course_id, course_title, lifecycle_status)
                        .delegated_permissions
                        .insert(delegation.permission.clone());
                }
            }
            _ => {}
        }
    }

    Ok(CurrentSessionResponse {
        user: user.into(),
        platform: platform.into(),
        organizations: organizations
            .into_values()
            .map(OrganizationSessionScope::from)
            .collect(),
        courses: courses
            .into_values()
            .map(CourseSessionScope::from)
            .collect(),
        delegated_permissions: delegations
            .into_iter()
            .map(|delegation| {
                delegated_permission_session(delegation, &organization_labels, &course_labels)
            })
            .collect(),
    })
}

fn organization_builder(
    organizations: &mut BTreeMap<i32, OrganizationScopeBuilder>,
    id: i32,
    name: String,
) -> &mut OrganizationScopeBuilder {
    organizations
        .entry(id)
        .or_insert_with(|| OrganizationScopeBuilder {
            id,
            name,
            roles: BTreeSet::new(),
            direct_permissions: BTreeSet::new(),
            delegated_permissions: BTreeSet::new(),
        })
}

fn course_builder(
    courses: &mut BTreeMap<i32, CourseScopeBuilder>,
    id: i32,
    title: String,
    lifecycle_status: String,
) -> &mut CourseScopeBuilder {
    courses.entry(id).or_insert_with(|| CourseScopeBuilder {
        id,
        title,
        lifecycle_status,
        roles: BTreeSet::new(),
        direct_permissions: BTreeSet::new(),
        delegated_permissions: BTreeSet::new(),
    })
}

fn sorted_vec(values: BTreeSet<String>) -> Vec<String> {
    values.into_iter().collect()
}

fn effective_permissions(
    direct_permissions: &BTreeSet<String>,
    delegated_permissions: &BTreeSet<String>,
) -> Vec<String> {
    direct_permissions
        .union(delegated_permissions)
        .cloned()
        .collect()
}

impl From<User> for CurrentSessionUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            kyc_verified: user.kyc_verified,
        }
    }
}

impl From<PlatformScopeBuilder> for PlatformSessionScope {
    fn from(builder: PlatformScopeBuilder) -> Self {
        Self {
            roles: sorted_vec(builder.roles),
            effective_permissions: effective_permissions(
                &builder.direct_permissions,
                &builder.delegated_permissions,
            ),
            direct_permissions: sorted_vec(builder.direct_permissions),
            delegated_permissions: sorted_vec(builder.delegated_permissions),
        }
    }
}

impl From<OrganizationScopeBuilder> for OrganizationSessionScope {
    fn from(builder: OrganizationScopeBuilder) -> Self {
        Self {
            id: builder.id,
            name: builder.name,
            roles: sorted_vec(builder.roles),
            effective_permissions: effective_permissions(
                &builder.direct_permissions,
                &builder.delegated_permissions,
            ),
            direct_permissions: sorted_vec(builder.direct_permissions),
            delegated_permissions: sorted_vec(builder.delegated_permissions),
        }
    }
}

impl From<CourseScopeBuilder> for CourseSessionScope {
    fn from(builder: CourseScopeBuilder) -> Self {
        Self {
            id: builder.id,
            title: builder.title,
            lifecycle_status: builder.lifecycle_status,
            roles: sorted_vec(builder.roles),
            effective_permissions: effective_permissions(
                &builder.direct_permissions,
                &builder.delegated_permissions,
            ),
            direct_permissions: sorted_vec(builder.direct_permissions),
            delegated_permissions: sorted_vec(builder.delegated_permissions),
        }
    }
}

fn delegated_permission_session(
    delegation: DelegatedPermission,
    organization_labels: &BTreeMap<i32, String>,
    course_labels: &BTreeMap<i32, (String, String)>,
) -> DelegatedPermissionSession {
    let (course_title, course_lifecycle_status) = delegation
        .course_id
        .and_then(|course_id| course_labels.get(&course_id).cloned())
        .map(|(title, lifecycle_status)| (Some(title), Some(lifecycle_status)))
        .unwrap_or((None, None));

    DelegatedPermissionSession {
        id: delegation.id,
        grantor_user_id: delegation.grantor_user_id,
        permission: delegation.permission,
        scope_type: delegation.scope_type,
        organization_id: delegation.organization_id,
        organization_name: delegation
            .organization_id
            .and_then(|organization_id| organization_labels.get(&organization_id).cloned()),
        course_id: delegation.course_id,
        course_title,
        course_lifecycle_status,
        expires_at: delegation.expires_at,
    }
}
