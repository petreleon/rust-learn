use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses_organizations, delegated_permissions, organization_roles, organizations,
    role_permission_organization, user_role_organization, users,
};
use crate::db::DbPool;
use crate::models::course::Course;
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::organization::{NewOrganization, Organization, UpdateOrganization};
use crate::repositories::organization_repository::assign_role_to_user_in_organization;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use chrono::{DateTime, Utc};
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

#[derive(Debug)]
struct OrganizationMemberBuilder {
    id: i32,
    name: String,
    email: String,
    email_verified: bool,
    kyc_verified: bool,
    joined_at: chrono::NaiveDateTime,
    roles: BTreeSet<String>,
    direct_permissions: BTreeSet<String>,
    delegated_permissions: BTreeSet<String>,
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

pub async fn list_organization_members(
    conn: &mut diesel_async::AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    query: OrganizationMemberListQuery,
) -> Result<OrganizationMemberListResponse, OrganizationMemberListError> {
    let organization = organizations::table
        .find(organization_id)
        .first::<Organization>(conn)
        .await
        .map_err(OrganizationMemberListError::from)?;

    if !user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?
    {
        return Err(OrganizationMemberListError::PermissionDenied);
    }

    let operator_permissions =
        build_member_operator_permissions(conn, actor_user_id, organization_id).await?;
    let mut members = build_organization_member_builders(conn, organization_id).await?;
    attach_member_permissions(conn, organization_id, &mut members).await?;
    attach_member_delegations(conn, organization_id, &mut members).await?;

    let mut items = members
        .into_values()
        .map(OrganizationMemberListItem::from)
        .filter(|member| member_matches_query(member, &query))
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| left.email.to_lowercase().cmp(&right.email.to_lowercase()))
            .then_with(|| left.id.cmp(&right.id))
    });

    let total = items.len() as i64;
    let members = items
        .into_iter()
        .skip(query.offset as usize)
        .take(query.limit as usize)
        .collect::<Vec<_>>();

    Ok(OrganizationMemberListResponse {
        organization: OrganizationMemberListOrganization {
            id: organization.id,
            name: organization.name,
        },
        members,
        total,
        limit: query.limit,
        offset: query.offset,
        search: query.search,
        role: query.role,
        permission: query.permission,
        operator_permissions,
    })
}

pub struct CreateOrganizationDto {
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
    pub course_ids: Option<Vec<i32>>,
}

pub async fn create_organization(
    pool: &DbPool,
    req: CreateOrganizationDto,
) -> Result<Organization, String> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| "Failed to get DB connection".to_string())?;

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let new_org = NewOrganization {
                name: req.name,
                website_link: req.website_link,
                profile_url: req.profile_url,
            };

            let org = diesel::insert_into(organizations::table)
                .values(&new_org)
                .get_result::<Organization>(conn)
                .await?;

            if let Some(course_ids) = &req.course_ids {
                for (index, course_id) in course_ids.iter().enumerate() {
                    let new_link = NewCourseOrganization {
                        course_id: *course_id,
                        organization_id: org.id,
                        order: index as i32,
                    };
                    diesel::insert_into(courses_organizations::table)
                        .values(&new_link)
                        .execute(conn)
                        .await?;
                }
            }

            Ok(org)
        })
    })
    .await
    .map_err(|e| format!("DB error: {}", e))
}

pub async fn update_organization(
    pool: &DbPool,
    org_id: i32,
    req: UpdateOrganization,
) -> Result<Organization, diesel::result::Error> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| diesel::result::Error::NotFound)?; // Simplified
    diesel::update(organizations::table.find(org_id))
        .set(&req)
        .get_result::<Organization>(&mut conn)
        .await
}

pub async fn delete_organization(pool: &DbPool, org_id: i32) -> Result<usize, String> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| "Failed to get DB connection".to_string())?;
    diesel::delete(organizations::table.find(org_id))
        .execute(&mut conn)
        .await
        .map_err(|e| format!("DB error: {}", e))
}

pub async fn get_organization_courses(pool: &DbPool, org_id: i32) -> Result<Vec<Course>, String> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| "Failed to get DB connection".to_string())?;

    courses_organizations::table
        .filter(courses_organizations::organization_id.eq(org_id))
        .inner_join(crate::db::schema::courses::table)
        .select(crate::db::schema::courses::all_columns)
        .load::<Course>(&mut conn)
        .await
        .map_err(|e| format!("DB error: {}", e))
}

pub async fn assign_role(
    pool: &DbPool,
    requester_id: i32,
    target_user_id: i32,
    org_id: i32,
    role_name: &str,
) -> Result<(), String> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| "Failed to get DB connection".to_string())?;
    match assign_role_to_user_in_organization(
        &mut conn,
        requester_id,
        target_user_id,
        org_id,
        role_name,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::RollbackTransaction) => {
            Err("Hierarchy check failed".to_string())
        }
        Err(diesel::result::Error::NotFound) => Err("Role or User not found".to_string()),
        Err(e) => Err(format!("Error assigning role: {}", e)),
    }
}

async fn build_organization_member_builders(
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

async fn attach_member_permissions(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
    members: &mut BTreeMap<i32, OrganizationMemberBuilder>,
) -> QueryResult<()> {
    if members.is_empty() {
        return Ok(());
    }

    let rows = user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::organization_id.eq(organization_id))
        .filter(
            role_permission_organization::organization_id
                .is_null()
                .or(role_permission_organization::organization_id.eq(Some(organization_id))),
        )
        .select((
            user_role_organization::user_id,
            role_permission_organization::permission,
        ))
        .load::<(Option<i32>, String)>(conn)
        .await?;

    for (user_id, permission) in rows {
        if let Some(member) = user_id.and_then(|id| members.get_mut(&id)) {
            member.direct_permissions.insert(permission);
        }
    }

    Ok(())
}

async fn attach_member_delegations(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
    members: &mut BTreeMap<i32, OrganizationMemberBuilder>,
) -> QueryResult<()> {
    if members.is_empty() {
        return Ok(());
    }

    let now: DateTime<Utc> = Utc::now();
    let rows = delegated_permissions::table
        .filter(delegated_permissions::scope_type.eq("organization"))
        .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
        .filter(delegated_permissions::course_id.is_null())
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .select((
            delegated_permissions::grantee_user_id,
            delegated_permissions::permission,
        ))
        .load::<(i32, String)>(conn)
        .await?;

    for (user_id, permission) in rows {
        if let Some(member) = members.get_mut(&user_id) {
            member.delegated_permissions.insert(permission);
        }
    }

    Ok(())
}

async fn build_member_operator_permissions(
    conn: &mut diesel_async::AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationMemberOperatorPermissions, OrganizationMemberListError> {
    Ok(OrganizationMemberOperatorPermissions {
        can_view_members: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORGANIZATION,
        )
        .await?,
        can_invite_members: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::INVITE_USER_TO_ORGANIZATION,
        )
        .await?,
        can_manage_members: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_MEMBERS,
        )
        .await?,
        can_assign_roles: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::ASSIGN_ROLES_TO_ORG_USERS,
        )
        .await?,
        can_manage_settings: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_SETTINGS,
        )
        .await?,
    })
}

async fn user_has_platform_or_organization_permission(
    conn: &mut diesel_async::AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> QueryResult<bool> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, user_id, &permission_name).await? {
        return Ok(true);
    }

    user_permission_organization_request(conn, user_id, organization_id, &permission_name).await
}

fn member_matches_query(
    member: &OrganizationMemberListItem,
    query: &OrganizationMemberListQuery,
) -> bool {
    let matches_search = query
        .search
        .as_deref()
        .map(|search| {
            let normalized = search.to_lowercase();
            member.name.to_lowercase().contains(&normalized)
                || member.email.to_lowercase().contains(&normalized)
                || member
                    .roles
                    .iter()
                    .any(|role| role.to_lowercase().contains(&normalized))
                || member
                    .effective_permissions
                    .iter()
                    .any(|permission| permission.to_lowercase().contains(&normalized))
        })
        .unwrap_or(true);

    let matches_role = query
        .role
        .as_deref()
        .map(|role| {
            member
                .roles
                .iter()
                .any(|member_role| member_role.eq_ignore_ascii_case(role))
        })
        .unwrap_or(true);

    let matches_permission = query
        .permission
        .as_deref()
        .map(|permission| {
            member
                .effective_permissions
                .iter()
                .any(|member_permission| member_permission.eq_ignore_ascii_case(permission))
        })
        .unwrap_or(true);

    matches_search && matches_role && matches_permission
}

fn normalize_query_value(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn sorted_vec(values: BTreeSet<String>) -> Vec<String> {
    values.into_iter().collect()
}

impl From<OrganizationMemberBuilder> for OrganizationMemberListItem {
    fn from(member: OrganizationMemberBuilder) -> Self {
        let direct_permissions = sorted_vec(member.direct_permissions);
        let delegated_permissions = sorted_vec(member.delegated_permissions);
        let effective_permissions = direct_permissions
            .iter()
            .chain(delegated_permissions.iter())
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let direct_permission_count = direct_permissions.len();
        let delegated_permission_count = delegated_permissions.len();
        let effective_permission_count = effective_permissions.len();

        Self {
            id: member.id,
            name: member.name,
            email: member.email,
            email_verified: member.email_verified,
            kyc_verified: member.kyc_verified,
            joined_at: member.joined_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            roles: sorted_vec(member.roles),
            direct_permissions,
            delegated_permissions,
            effective_permissions,
            direct_permission_count,
            delegated_permission_count,
            effective_permission_count,
        }
    }
}
