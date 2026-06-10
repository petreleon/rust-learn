use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, courses_organizations, delegated_permissions, organization_roles, organizations,
    reward_candidates, role_permission_organization, teacher_applications, user_role_organization,
    users, wallets,
};
use crate::db::DbPool;
use crate::models::course::{
    Course, COURSE_STATUS_APPROVED, COURSE_STATUS_ARCHIVED, COURSE_STATUS_DRAFT,
    COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED, COURSE_STATUS_SUBMITTED,
    COURSE_STATUS_SUSPENDED,
};
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::organization::{NewOrganization, Organization, UpdateOrganization};
use crate::models::reward_candidate::{REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION};
use crate::models::teacher_application::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::repositories::organization_repository::assign_role_to_user_in_organization;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::services::reporting_service::organization_reward_dashboard;
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

pub async fn get_organization_dashboard(
    conn: &mut diesel_async::AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationDashboardResponse, OrganizationDashboardError> {
    let organization = organizations::table
        .find(organization_id)
        .first::<Organization>(conn)
        .await
        .map_err(OrganizationDashboardError::from)?;

    let can_view_dashboard =
        user_has_organization_dashboard_access(conn, actor_user_id, organization_id)
            .await
            .map_err(OrganizationDashboardError::from)?;
    if !can_view_dashboard {
        return Err(OrganizationDashboardError::PermissionDenied);
    }

    let can_view_members = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_view_courses = can_view_members;
    let can_view_reports = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORG_REWARD_REPORTS,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_view_teacher_applications = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORG_TEACHER_APPLICATIONS,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_nominate_teachers = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_manage_wallets = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::MANAGE_ORG_WALLETS,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_manage_reward_budget = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::MANAGE_ORG_REWARD_BUDGET,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;

    let operator_permissions = OrganizationDashboardOperatorPermissions {
        can_view_dashboard,
        can_view_members,
        can_view_courses,
        can_view_reports,
        can_view_teacher_applications,
        can_nominate_teachers,
        can_manage_wallets,
        can_manage_reward_budget,
    };

    let members = if can_view_members {
        organization_dashboard_member_summary(conn, organization_id).await?
    } else {
        gated_member_summary()
    };
    let courses = if can_view_courses {
        organization_dashboard_course_summary(conn, organization_id).await?
    } else {
        gated_course_summary()
    };
    let teacher_applications = if can_view_teacher_applications || can_nominate_teachers {
        organization_dashboard_teacher_application_summary(conn, organization_id).await?
    } else {
        gated_teacher_application_summary()
    };
    let rewards = if can_view_reports {
        organization_dashboard_reward_summary(conn, organization_id).await?
    } else {
        gated_reward_summary()
    };
    let wallet = if can_view_reports || can_manage_wallets || can_manage_reward_budget {
        organization_dashboard_wallet_summary(conn, organization_id).await?
    } else {
        gated_wallet_summary()
    };

    let alerts = organization_dashboard_alerts(
        organization_id,
        &courses,
        &teacher_applications,
        &rewards,
        &wallet,
        &operator_permissions,
    );
    let health = OrganizationDashboardHealth {
        status: if alerts.iter().any(|alert| alert.severity == "warning") {
            "attention".to_string()
        } else {
            "active".to_string()
        },
        alert_count: alerts.len(),
    };

    Ok(OrganizationDashboardResponse {
        organization: OrganizationDashboardOrganization {
            id: organization.id,
            name: organization.name,
        },
        health,
        members,
        courses,
        teacher_applications,
        rewards,
        wallet,
        operator_permissions,
        alerts,
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
        Ok(_) => {
            log_organization_member_event(&mut conn, org_id, Some(requester_id), target_user_id, "role_assigned", Some(role_name), None).await.ok();
            Ok(())
        }
        Err(diesel::result::Error::RollbackTransaction) => {
            Err("Hierarchy check failed".to_string())
        }
        Err(diesel::result::Error::NotFound) => Err("Role or User not found".to_string()),
        Err(e) => Err(format!("Error assigning role: {}", e)),
    }
}

pub async fn remove_organization_member(
    pool: &DbPool,
    org_id: i32,
    target_user_id: i32,
) -> Result<(), String> {
    use crate::db::schema::user_role_organization;
    let mut conn = pool
        .get()
        .await
        .map_err(|_| "Failed to get DB connection".to_string())?;

    let deleted = diesel::delete(
        user_role_organization::table
            .filter(user_role_organization::organization_id.eq(org_id))
            .filter(user_role_organization::user_id.eq(target_user_id)),
    )
    .execute(&mut conn)
    .await
    .map_err(|e| format!("Error removing member: {}", e))?;

    if deleted == 0 {
        return Err("User not found in organization".to_string());
    }

    log_organization_member_event(&mut conn, org_id, None, target_user_id, "member_removed", None, None).await.ok();
    Ok(())
}

async fn log_organization_member_event(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
    actor_user_id: Option<i32>,
    target_user_id: i32,
    event_type: &str,
    role_name: Option<&str>,
    reason: Option<&str>,
) -> QueryResult<()> {
    use crate::db::schema::organization_member_audit_events;
    use crate::models::organization_member_audit_event::NewOrganizationMemberAuditEvent;

    diesel::insert_into(organization_member_audit_events::table)
        .values(NewOrganizationMemberAuditEvent {
            organization_id,
            actor_user_id,
            target_user_id,
            event_type: event_type.to_string(),
            role_name: role_name.map(|s| s.to_string()),
            reason: reason.map(|s| s.to_string()),
        })
        .execute(conn)
        .await?;
    Ok(())
}

async fn organization_dashboard_member_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardMemberSummary, OrganizationDashboardError> {
    let members = build_organization_member_builders(conn, organization_id).await?;
    let total = members.len() as i64;
    let verified_email_count = members
        .values()
        .filter(|member| member.email_verified)
        .count() as i64;
    let kyc_ready_count = members
        .values()
        .filter(|member| member.kyc_verified)
        .count() as i64;

    let now: DateTime<Utc> = Utc::now();
    let delegated_permission_count = delegated_permissions::table
        .filter(delegated_permissions::scope_type.eq("organization"))
        .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
        .filter(delegated_permissions::course_id.is_null())
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .count()
        .get_result::<i64>(conn)
        .await?;

    Ok(OrganizationDashboardMemberSummary {
        available: true,
        missing_permissions: vec![],
        total,
        verified_email_count,
        kyc_ready_count,
        delegated_permission_count,
    })
}

async fn organization_dashboard_course_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardCourseSummary, OrganizationDashboardError> {
    let statuses = courses_organizations::table
        .inner_join(courses::table.on(courses_organizations::course_id.eq(courses::id)))
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses::lifecycle_status)
        .load::<String>(conn)
        .await?;

    let mut summary = OrganizationDashboardCourseSummary {
        available: true,
        missing_permissions: vec![],
        total: statuses.len() as i64,
        draft: 0,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        published: 0,
        suspended: 0,
        archived: 0,
    };

    for status in statuses {
        match status.as_str() {
            COURSE_STATUS_DRAFT => summary.draft += 1,
            COURSE_STATUS_SUBMITTED => summary.submitted += 1,
            COURSE_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            COURSE_STATUS_APPROVED => summary.approved += 1,
            COURSE_STATUS_PUBLISHED => summary.published += 1,
            COURSE_STATUS_SUSPENDED => summary.suspended += 1,
            COURSE_STATUS_ARCHIVED => summary.archived += 1,
            _ => {}
        }
    }

    Ok(summary)
}

async fn organization_dashboard_teacher_application_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardTeacherApplicationSummary, OrganizationDashboardError> {
    let statuses = teacher_applications::table
        .filter(
            teacher_applications::organization_sponsor_id
                .eq(Some(organization_id))
                .or(teacher_applications::requested_organization_id.eq(Some(organization_id))),
        )
        .select(teacher_applications::status)
        .load::<String>(conn)
        .await?;

    let mut summary = OrganizationDashboardTeacherApplicationSummary {
        available: true,
        missing_permissions: vec![],
        ..Default::default()
    };
    summary.total = statuses.len() as i64;
    for status in statuses {
        match status.as_str() {
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted += 1,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved += 1,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected += 1,
            _ => {}
        }
    }

    Ok(summary)
}

async fn organization_dashboard_reward_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardRewardSummary, OrganizationDashboardError> {
    let reward_dashboard = organization_reward_dashboard(conn, organization_id, None, None).await?;
    let course_ids = courses_organizations::table
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await?;

    let statuses = if course_ids.is_empty() {
        Vec::new()
    } else {
        reward_candidates::table
            .filter(reward_candidates::course_id.eq_any(&course_ids))
            .select(reward_candidates::status)
            .load::<String>(conn)
            .await?
    };
    let failed_count = statuses
        .iter()
        .filter(|status| status.as_str() == REWARD_STATUS_FAILED)
        .count() as i64;
    let needs_reconciliation_count = statuses
        .iter()
        .filter(|status| status.as_str() == REWARD_STATUS_NEEDS_RECONCILIATION)
        .count() as i64;

    Ok(OrganizationDashboardRewardSummary {
        available: true,
        missing_permissions: vec![],
        reward_candidate_count: reward_dashboard.course_reward_count,
        approved_reward_count: reward_dashboard.approved_reward_count,
        approved_amount_total: reward_dashboard.approved_amount_total,
        failed_count,
        needs_reconciliation_count,
    })
}

async fn organization_dashboard_wallet_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardWalletSummary, OrganizationDashboardError> {
    let balances = wallets::table
        .filter(wallets::organization_id.eq(Some(organization_id)))
        .filter(wallets::user_id.is_null())
        .select(wallets::value)
        .load::<BigDecimal>(conn)
        .await?;
    let balance_total = balances
        .iter()
        .cloned()
        .fold(BigDecimal::from(0), |total, balance| total + balance);

    Ok(OrganizationDashboardWalletSummary {
        available: true,
        missing_permissions: vec![],
        wallet_count: balances.len() as i64,
        balance_total: balance_total.to_string(),
    })
}

fn gated_member_summary() -> OrganizationDashboardMemberSummary {
    OrganizationDashboardMemberSummary {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORGANIZATION.to_string()],
        total: 0,
        verified_email_count: 0,
        kyc_ready_count: 0,
        delegated_permission_count: 0,
    }
}

fn gated_course_summary() -> OrganizationDashboardCourseSummary {
    OrganizationDashboardCourseSummary {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORGANIZATION.to_string()],
        total: 0,
        draft: 0,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        published: 0,
        suspended: 0,
        archived: 0,
    }
}

fn gated_teacher_application_summary() -> OrganizationDashboardTeacherApplicationSummary {
    OrganizationDashboardTeacherApplicationSummary {
        available: false,
        missing_permissions: vec![
            Permissions::VIEW_ORG_TEACHER_APPLICATIONS.to_string(),
            Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW.to_string(),
        ],
        ..Default::default()
    }
}

fn gated_reward_summary() -> OrganizationDashboardRewardSummary {
    OrganizationDashboardRewardSummary {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORG_REWARD_REPORTS.to_string()],
        reward_candidate_count: 0,
        approved_reward_count: 0,
        approved_amount_total: "0".to_string(),
        failed_count: 0,
        needs_reconciliation_count: 0,
    }
}

fn gated_wallet_summary() -> OrganizationDashboardWalletSummary {
    OrganizationDashboardWalletSummary {
        available: false,
        missing_permissions: vec![
            Permissions::MANAGE_ORG_WALLETS.to_string(),
            Permissions::MANAGE_ORG_REWARD_BUDGET.to_string(),
            Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
        ],
        wallet_count: 0,
        balance_total: "0".to_string(),
    }
}

fn organization_dashboard_alerts(
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

async fn user_has_organization_dashboard_access(
    conn: &mut diesel_async::AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
) -> QueryResult<bool> {
    if user_has_platform_or_organization_permission(
        conn,
        user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?
    {
        return Ok(true);
    }

    let has_role = select(exists(
        user_role_organization::table
            .filter(user_role_organization::user_id.eq(Some(user_id)))
            .filter(user_role_organization::organization_id.eq(Some(organization_id))),
    ))
    .get_result::<bool>(conn)
    .await?;
    if has_role {
        return Ok(true);
    }

    let now: DateTime<Utc> = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(user_id))
            .filter(delegated_permissions::scope_type.eq("organization"))
            .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result::<bool>(conn)
    .await
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
