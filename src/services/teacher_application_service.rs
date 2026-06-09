use crate::config::constants::permissions::Permissions;
use crate::config::constants::roles::Roles;
use crate::db::schema::{
    courses, organizations, teacher_application_audit_events, teacher_applications,
    user_role_course, user_role_organization, user_role_platform, users,
};
use crate::models::role::{CourseRole, OrganizationRole, PlatformRole};
use crate::models::teacher_application::{
    NewTeacherApplication, NewTeacherApplicationAuditEvent, TeacherApplication,
    TeacherApplicationAuditEvent, TEACHER_APPLICATION_SCOPE_COURSE,
    TEACHER_APPLICATION_SCOPE_ORGANIZATION, TEACHER_APPLICATION_SCOPE_PLATFORM,
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::models::user_role_course::UserRoleCourse;
use crate::models::user_role_organization::UserRoleOrganization;
use crate::models::user_role_platform::UserRolePlatform;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::teacher_application_repository::{self, TeacherApplicationFilter};
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

#[derive(Debug, Clone, Deserialize)]
pub struct TeacherApplicationDecisionRequest {
    pub status: String,
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationSelfResponse {
    pub application: Option<TeacherApplication>,
    pub audit_events: Vec<TeacherApplicationAuditEvent>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListTeacherApplicationsRequest {
    pub status: Option<String>,
    pub applicant_user_id: Option<i32>,
    pub organization_sponsor_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
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

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationTeacherApplicationItem {
    pub id: i64,
    pub applicant: TeacherApplicationUserSummary,
    pub requested_scope: String,
    pub requested_organization: Option<TeacherApplicationOrganizationSummary>,
    pub requested_course: Option<TeacherApplicationCourseSummary>,
    pub sponsored_by_this_organization: bool,
    pub requested_for_this_organization: bool,
    pub experience_summary: String,
    pub portfolio_links: Vec<String>,
    pub status: String,
    pub reviewer: Option<TeacherApplicationUserSummary>,
    pub decision_reason: Option<String>,
    pub audit: TeacherApplicationAuditSummary,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub decided_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationUserSummary {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationOrganizationSummary {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationCourseSummary {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TeacherApplicationAuditSummary {
    pub event_count: usize,
    pub latest_event_type: Option<String>,
    pub latest_event_at: Option<chrono::DateTime<chrono::Utc>>,
    pub latest_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TeacherApplicationDashboardSummary {
    pub approved: i64,
    pub needs_changes: i64,
    pub rejected: i64,
    pub submitted: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationTeacherApplicationPermissions {
    pub can_view_applications: bool,
    pub can_nominate_teachers: bool,
}

pub async fn submit_application(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: SubmitTeacherApplicationRequest,
) -> Result<TeacherApplication, TeacherApplicationError> {
    ensure_platform_permission(conn, actor_user_id, Permissions::SUBMIT_TEACHER_APPLICATION)
        .await?;

    let new_application = build_new_application(actor_user_id, request, None)?;
    if let Some(existing) = find_idempotent_application(conn, &new_application).await? {
        log::info!(
            "event=teacher_application_idempotent_replay application_id={} actor_user_id={} applicant_user_id={} status={} requested_scope={} idempotency_key={}",
            existing.id,
            actor_user_id,
            existing.applicant_user_id,
            existing.status,
            existing.requested_scope,
            existing.idempotency_key.as_deref().unwrap_or("none")
        );
        return Ok(existing);
    }
    ensure_no_blocking_application(conn, &new_application).await?;

    let application = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let application =
                    teacher_application_repository::create_application(conn, new_application)
                        .await?;
                teacher_application_repository::create_audit_event(
                    conn,
                    NewTeacherApplicationAuditEvent {
                        application_id: application.id,
                        actor_user_id: Some(actor_user_id),
                        event_type: "submitted".to_string(),
                        from_status: None,
                        to_status: application.status.clone(),
                        reason: None,
                    },
                )
                .await?;
                Ok(application)
            })
        })
        .await
        .map_err(TeacherApplicationError::from)?;

    log::info!(
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition=submitted from_status=none to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
        application.id,
        actor_user_id,
        application.applicant_user_id,
        application.status,
        application.requested_scope,
        application.organization_sponsor_id,
        application.requested_organization_id,
        application.requested_course_id
    );

    Ok(application)
}

pub async fn nominate_application(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    request: OrganizationTeacherNominationRequest,
) -> Result<TeacherApplication, TeacherApplicationError> {
    ensure_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
    )
    .await?;

    let requested_scope = request
        .requested_scope
        .clone()
        .unwrap_or_else(|| TEACHER_APPLICATION_SCOPE_ORGANIZATION.to_string());
    let submit_request = SubmitTeacherApplicationRequest {
        requested_scope,
        requested_organization_id: Some(organization_id),
        requested_course_id: request.requested_course_id,
        experience_summary: request.experience_summary,
        organization_sponsor_id: Some(organization_id),
        portfolio_links: request.portfolio_links,
        idempotency_key: request.idempotency_key,
    };
    let new_application = build_new_application(
        request.applicant_user_id,
        submit_request,
        Some(organization_id),
    )?;
    if let Some(existing) = find_idempotent_application(conn, &new_application).await? {
        log::info!(
            "event=teacher_application_idempotent_replay application_id={} actor_user_id={} applicant_user_id={} status={} requested_scope={} idempotency_key={}",
            existing.id,
            actor_user_id,
            existing.applicant_user_id,
            existing.status,
            existing.requested_scope,
            existing.idempotency_key.as_deref().unwrap_or("none")
        );
        return Ok(existing);
    }
    ensure_no_blocking_application(conn, &new_application).await?;

    let application = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let application =
                    teacher_application_repository::create_application(conn, new_application)
                        .await?;
                teacher_application_repository::create_audit_event(
                    conn,
                    NewTeacherApplicationAuditEvent {
                        application_id: application.id,
                        actor_user_id: Some(actor_user_id),
                        event_type: "organization_nominated".to_string(),
                        from_status: None,
                        to_status: application.status.clone(),
                        reason: None,
                    },
                )
                .await?;
                Ok(application)
            })
        })
        .await
        .map_err(TeacherApplicationError::from)?;

    log::info!(
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition=organization_nominated from_status=none to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
        application.id,
        actor_user_id,
        application.applicant_user_id,
        application.status,
        application.requested_scope,
        application.organization_sponsor_id,
        application.requested_organization_id,
        application.requested_course_id
    );

    Ok(application)
}

pub async fn get_my_application(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<TeacherApplicationSelfResponse, TeacherApplicationError> {
    let application =
        teacher_application_repository::find_latest_application_for_applicant(conn, actor_user_id)
            .await
            .map_err(TeacherApplicationError::from)?;

    let audit_events = match application.as_ref() {
        Some(application) => {
            teacher_application_repository::list_audit_events(conn, application.id)
                .await
                .map_err(TeacherApplicationError::from)?
        }
        None => Vec::new(),
    };

    Ok(TeacherApplicationSelfResponse {
        application,
        audit_events,
    })
}

pub async fn list_applications(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: ListTeacherApplicationsRequest,
) -> Result<Vec<TeacherApplication>, TeacherApplicationError> {
    ensure_platform_permission(
        conn,
        actor_user_id,
        Permissions::REVIEW_TEACHER_APPLICATIONS,
    )
    .await?;

    let status = match request.status {
        Some(status) => Some(normalize_status(&status)?),
        None => None,
    };

    teacher_application_repository::list_applications(
        conn,
        TeacherApplicationFilter {
            status,
            applicant_user_id: request.applicant_user_id,
            organization_sponsor_id: request.organization_sponsor_id,
            limit: request.limit,
            offset: request.offset,
        },
    )
    .await
    .map_err(TeacherApplicationError::from)
}

pub async fn list_organization_applications(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    request: OrganizationTeacherApplicationsRequest,
) -> Result<OrganizationTeacherApplicationsResponse, TeacherApplicationError> {
    let organization = organizations::table
        .find(organization_id)
        .select((organizations::id, organizations::name))
        .first::<(i32, String)>(conn)
        .await
        .map_err(TeacherApplicationError::from)?;

    let can_view_applications = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORG_TEACHER_APPLICATIONS,
    )
    .await?;
    let can_nominate_teachers = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
    )
    .await?;

    if !can_view_applications && !can_nominate_teachers {
        return Err(TeacherApplicationError::PermissionDenied(
            Permissions::VIEW_ORG_TEACHER_APPLICATIONS.to_string(),
        ));
    }

    let status = match request.status {
        Some(status) => Some(normalize_status(&status)?),
        None => None,
    };
    let search = normalize_optional_text(request.search);
    let limit = request.limit.unwrap_or(25).clamp(1, 100);
    let offset = request.offset.unwrap_or(0).max(0);

    let applications = teacher_applications::table
        .filter(
            teacher_applications::organization_sponsor_id
                .eq(Some(organization_id))
                .or(teacher_applications::requested_organization_id.eq(Some(organization_id))),
        )
        .order(teacher_applications::created_at.desc())
        .then_order_by(teacher_applications::id.desc())
        .load::<TeacherApplication>(conn)
        .await
        .map_err(TeacherApplicationError::from)?;

    let summary = teacher_application_summary(&applications);
    let context = build_organization_application_context(conn, &applications).await?;
    let mut items = applications
        .iter()
        .map(|application| organization_application_item(application, organization_id, &context))
        .collect::<Vec<_>>();

    if let Some(status) = status.as_deref() {
        items.retain(|application| application.status == status);
    }
    if let Some(search) = search.as_deref() {
        let normalized = search.to_lowercase();
        items.retain(|application| {
            organization_application_matches_search(application, &normalized)
        });
    }

    let total = items.len() as i64;
    let applications = items
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<_>>();

    Ok(OrganizationTeacherApplicationsResponse {
        organization: OrganizationTeacherApplicationsOrganization {
            id: organization.0,
            name: organization.1,
        },
        applications,
        summary,
        operator_permissions: OrganizationTeacherApplicationPermissions {
            can_view_applications,
            can_nominate_teachers,
        },
        total,
        limit,
        offset,
        status,
        search,
    })
}

pub async fn decide_application(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    application_id: i64,
    request: TeacherApplicationDecisionRequest,
) -> Result<TeacherApplication, TeacherApplicationError> {
    let target_status = normalize_decision_status(&request.status)?;
    ensure_decision_permission(conn, actor_user_id, &target_status).await?;

    let application = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let current =
                    teacher_application_repository::find_application(conn, application_id).await?;
                if current.status == TEACHER_APPLICATION_STATUS_APPROVED
                    || current.status == TEACHER_APPLICATION_STATUS_REJECTED
                {
                    return Err(diesel::result::Error::RollbackTransaction);
                }

                let now = chrono::Utc::now();
                let updated = teacher_application_repository::update_application_decision(
                    conn,
                    application_id,
                    actor_user_id,
                    &target_status,
                    request.decision_reason.as_deref(),
                    now,
                )
                .await?;
                if target_status == TEACHER_APPLICATION_STATUS_APPROVED {
                    assign_approved_teaching_bundle(conn, &updated).await?;
                }
                teacher_application_repository::create_audit_event(
                    conn,
                    NewTeacherApplicationAuditEvent {
                        application_id,
                        actor_user_id: Some(actor_user_id),
                        event_type: target_status.clone(),
                        from_status: Some(current.status),
                        to_status: target_status,
                        reason: request.decision_reason,
                    },
                )
                .await?;
                Ok(updated)
            })
        })
        .await
        .map_err(|error| match error {
            diesel::result::Error::RollbackTransaction => {
                TeacherApplicationError::InvalidTransition(
                    "final teacher applications cannot be changed".to_string(),
                )
            }
            other => TeacherApplicationError::from(other),
        })?;

    log::info!(
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition={} to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
        application.id,
        actor_user_id,
        application.applicant_user_id,
        application.status,
        application.status,
        application.requested_scope,
        application.organization_sponsor_id,
        application.requested_organization_id,
        application.requested_course_id
    );

    Ok(application)
}

pub async fn list_audit_events(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    application_id: i64,
) -> Result<Vec<TeacherApplicationAuditEvent>, TeacherApplicationError> {
    ensure_platform_permission(
        conn,
        actor_user_id,
        Permissions::REVIEW_TEACHER_APPLICATIONS,
    )
    .await?;

    teacher_application_repository::list_audit_events(conn, application_id)
        .await
        .map_err(TeacherApplicationError::from)
}

struct OrganizationApplicationContext {
    users: BTreeMap<i32, TeacherApplicationUserSummary>,
    organizations: BTreeMap<i32, String>,
    courses: BTreeMap<i32, String>,
    audits: BTreeMap<i64, TeacherApplicationAuditSummary>,
}

async fn build_organization_application_context(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<OrganizationApplicationContext, TeacherApplicationError> {
    let user_ids = applications
        .iter()
        .flat_map(|application| [Some(application.applicant_user_id), application.reviewer_id])
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let users = if user_ids.is_empty() {
        BTreeMap::new()
    } else {
        users::table
            .filter(users::id.eq_any(&user_ids))
            .select((users::id, users::name, users::email))
            .load::<(i32, String, String)>(conn)
            .await
            .map_err(TeacherApplicationError::from)?
            .into_iter()
            .map(|(id, name, email)| (id, TeacherApplicationUserSummary { id, name, email }))
            .collect()
    };

    let organization_ids = applications
        .iter()
        .flat_map(|application| {
            [
                application.requested_organization_id,
                application.organization_sponsor_id,
            ]
        })
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let organizations = if organization_ids.is_empty() {
        BTreeMap::new()
    } else {
        organizations::table
            .filter(organizations::id.eq_any(&organization_ids))
            .select((organizations::id, organizations::name))
            .load::<(i32, String)>(conn)
            .await
            .map_err(TeacherApplicationError::from)?
            .into_iter()
            .collect()
    };

    let course_ids = applications
        .iter()
        .filter_map(|application| application.requested_course_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let courses = if course_ids.is_empty() {
        BTreeMap::new()
    } else {
        courses::table
            .filter(courses::id.eq_any(&course_ids))
            .select((courses::id, courses::title))
            .load::<(i32, String)>(conn)
            .await
            .map_err(TeacherApplicationError::from)?
            .into_iter()
            .collect()
    };

    let application_ids = applications
        .iter()
        .map(|application| application.id)
        .collect::<Vec<_>>();
    let audits = if application_ids.is_empty() {
        BTreeMap::new()
    } else {
        let audit_events = teacher_application_audit_events::table
            .filter(teacher_application_audit_events::application_id.eq_any(&application_ids))
            .order(teacher_application_audit_events::created_at.asc())
            .then_order_by(teacher_application_audit_events::id.asc())
            .load::<TeacherApplicationAuditEvent>(conn)
            .await
            .map_err(TeacherApplicationError::from)?;
        build_audit_summaries(audit_events)
    };

    Ok(OrganizationApplicationContext {
        users,
        organizations,
        courses,
        audits,
    })
}

fn organization_application_item(
    application: &TeacherApplication,
    organization_id: i32,
    context: &OrganizationApplicationContext,
) -> OrganizationTeacherApplicationItem {
    OrganizationTeacherApplicationItem {
        id: application.id,
        applicant: context
            .users
            .get(&application.applicant_user_id)
            .cloned()
            .unwrap_or(TeacherApplicationUserSummary {
                id: application.applicant_user_id,
                name: "Unknown applicant".to_string(),
                email: "unknown@example.invalid".to_string(),
            }),
        requested_scope: application.requested_scope.clone(),
        requested_organization: application.requested_organization_id.map(|id| {
            TeacherApplicationOrganizationSummary {
                id,
                name: context
                    .organizations
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| format!("Organization {id}")),
            }
        }),
        requested_course: application.requested_course_id.map(|id| {
            TeacherApplicationCourseSummary {
                id,
                title: context
                    .courses
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| format!("Course {id}")),
            }
        }),
        sponsored_by_this_organization: application.organization_sponsor_id
            == Some(organization_id),
        requested_for_this_organization: application.requested_organization_id
            == Some(organization_id),
        experience_summary: application.experience_summary.clone(),
        portfolio_links: portfolio_links_from_json(&application.portfolio_links),
        status: application.status.clone(),
        reviewer: application
            .reviewer_id
            .and_then(|reviewer_id| context.users.get(&reviewer_id).cloned()),
        decision_reason: application.decision_reason.clone(),
        audit: context
            .audits
            .get(&application.id)
            .cloned()
            .unwrap_or_default(),
        created_at: application.created_at,
        updated_at: application.updated_at,
        decided_at: application.decided_at,
    }
}

fn build_audit_summaries(
    audit_events: Vec<TeacherApplicationAuditEvent>,
) -> BTreeMap<i64, TeacherApplicationAuditSummary> {
    let mut summaries = BTreeMap::<i64, TeacherApplicationAuditSummary>::new();
    for event in audit_events {
        let summary = summaries.entry(event.application_id).or_default();
        summary.event_count += 1;
        summary.latest_event_type = Some(event.event_type);
        summary.latest_event_at = Some(event.created_at);
        summary.latest_reason = event.reason;
    }
    summaries
}

fn teacher_application_summary(
    applications: &[TeacherApplication],
) -> TeacherApplicationDashboardSummary {
    let mut summary = TeacherApplicationDashboardSummary::default();
    for application in applications {
        summary.total += 1;
        match application.status.as_str() {
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved += 1,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected += 1,
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted += 1,
            _ => {}
        }
    }
    summary
}

fn organization_application_matches_search(
    application: &OrganizationTeacherApplicationItem,
    search: &str,
) -> bool {
    application.applicant.name.to_lowercase().contains(search)
        || application.applicant.email.to_lowercase().contains(search)
        || application
            .experience_summary
            .to_lowercase()
            .contains(search)
        || application.status.to_lowercase().contains(search)
        || application.requested_scope.to_lowercase().contains(search)
        || application
            .requested_course
            .as_ref()
            .is_some_and(|course| course.title.to_lowercase().contains(search))
        || application
            .requested_organization
            .as_ref()
            .is_some_and(|organization| organization.name.to_lowercase().contains(search))
}

async fn user_has_platform_or_organization_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, TeacherApplicationError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, user_id, &permission_name).await? {
        return Ok(true);
    }
    Ok(
        user_permission_organization_request(conn, user_id, organization_id, &permission_name)
            .await?,
    )
}

async fn assign_approved_teaching_bundle(
    conn: &mut AsyncPgConnection,
    application: &TeacherApplication,
) -> diesel::QueryResult<()> {
    match application.requested_scope.as_str() {
        TEACHER_APPLICATION_SCOPE_PLATFORM => {
            let role_id = PlatformRole::find_by_name(&Roles::TEACHER.to_string(), conn).await?;
            assign_platform_role_if_missing(conn, application.applicant_user_id, role_id).await
        }
        TEACHER_APPLICATION_SCOPE_ORGANIZATION => {
            let organization_id = application
                .requested_organization_id
                .or(application.organization_sponsor_id)
                .ok_or(diesel::result::Error::NotFound)?;
            let role_id = OrganizationRole::find_by_name("TEACHER", conn).await?;
            assign_organization_role_if_missing(
                conn,
                application.applicant_user_id,
                organization_id,
                role_id,
            )
            .await
        }
        TEACHER_APPLICATION_SCOPE_COURSE => {
            let course_id = application
                .requested_course_id
                .ok_or(diesel::result::Error::NotFound)?;
            let role_id = CourseRole::find_by_name("TEACHER", conn).await?;
            assign_course_role_if_missing(conn, application.applicant_user_id, course_id, role_id)
                .await
        }
        _ => Err(diesel::result::Error::NotFound),
    }
}

async fn assign_platform_role_if_missing(
    conn: &mut AsyncPgConnection,
    target_user_id: i32,
    platform_role_id: i32,
) -> diesel::QueryResult<()> {
    let already_assigned = diesel::select(diesel::dsl::exists(
        user_role_platform::table
            .filter(user_role_platform::user_id.eq(target_user_id))
            .filter(user_role_platform::platform_role_id.eq(platform_role_id)),
    ))
    .get_result::<bool>(conn)
    .await?;

    if !already_assigned {
        UserRolePlatform::assign(conn, target_user_id, platform_role_id).await?;
    }
    Ok(())
}

async fn assign_organization_role_if_missing(
    conn: &mut AsyncPgConnection,
    target_user_id: i32,
    organization_id: i32,
    organization_role_id: i32,
) -> diesel::QueryResult<()> {
    let already_assigned = diesel::select(diesel::dsl::exists(
        user_role_organization::table
            .filter(user_role_organization::user_id.eq(Some(target_user_id)))
            .filter(user_role_organization::organization_id.eq(Some(organization_id)))
            .filter(user_role_organization::organization_role_id.eq(Some(organization_role_id))),
    ))
    .get_result::<bool>(conn)
    .await?;

    if !already_assigned {
        UserRoleOrganization::assign(conn, target_user_id, organization_id, organization_role_id)
            .await?;
    }
    Ok(())
}

async fn assign_course_role_if_missing(
    conn: &mut AsyncPgConnection,
    target_user_id: i32,
    course_id: i32,
    course_role_id: i32,
) -> diesel::QueryResult<()> {
    let already_assigned = diesel::select(diesel::dsl::exists(
        user_role_course::table
            .filter(user_role_course::user_id.eq(Some(target_user_id)))
            .filter(user_role_course::course_id.eq(Some(course_id)))
            .filter(user_role_course::course_role_id.eq(Some(course_role_id))),
    ))
    .get_result::<bool>(conn)
    .await?;

    if !already_assigned {
        UserRoleCourse::assign(conn, target_user_id, course_id, course_role_id).await?;
    }
    Ok(())
}

async fn ensure_platform_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: Permissions,
) -> Result<(), TeacherApplicationError> {
    let permission_name = permission.to_string();
    match user_permission_platform_request(conn, user_id, &permission_name).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(TeacherApplicationError::PermissionDenied(permission_name)),
        Err(error) => Err(TeacherApplicationError::from(error)),
    }
}

async fn ensure_organization_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<(), TeacherApplicationError> {
    let permission_name = permission.to_string();
    match user_permission_organization_request(conn, user_id, organization_id, &permission_name)
        .await
    {
        Ok(true) => Ok(()),
        Ok(false) => Err(TeacherApplicationError::PermissionDenied(permission_name)),
        Err(error) => Err(TeacherApplicationError::from(error)),
    }
}

async fn ensure_decision_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    target_status: &str,
) -> Result<(), TeacherApplicationError> {
    let permission = match target_status {
        TEACHER_APPLICATION_STATUS_APPROVED => Permissions::APPROVE_TEACHER_APPLICATION,
        TEACHER_APPLICATION_STATUS_REJECTED => Permissions::REJECT_TEACHER_APPLICATION,
        TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => Permissions::REVIEW_TEACHER_APPLICATIONS,
        _ => {
            return Err(TeacherApplicationError::InvalidInput(
                "unsupported teacher application decision".to_string(),
            ))
        }
    };
    ensure_platform_permission(conn, user_id, permission).await
}

fn build_new_application(
    applicant_user_id: i32,
    request: SubmitTeacherApplicationRequest,
    forced_sponsor_id: Option<i32>,
) -> Result<NewTeacherApplication, TeacherApplicationError> {
    let requested_scope = normalize_scope(&request.requested_scope)?;
    let experience_summary = request.experience_summary.trim().to_string();
    if experience_summary.is_empty() {
        return Err(TeacherApplicationError::InvalidInput(
            "experience_summary is required".to_string(),
        ));
    }

    validate_requested_scope(
        &requested_scope,
        request.requested_organization_id,
        request.requested_course_id,
        request.organization_sponsor_id.or(forced_sponsor_id),
    )?;

    Ok(NewTeacherApplication {
        applicant_user_id,
        requested_scope,
        requested_organization_id: request.requested_organization_id,
        requested_course_id: request.requested_course_id,
        experience_summary,
        organization_sponsor_id: forced_sponsor_id.or(request.organization_sponsor_id),
        portfolio_links: json!(clean_portfolio_links(request.portfolio_links)),
        status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
        idempotency_key: normalize_idempotency_key(request.idempotency_key)?,
    })
}

async fn find_idempotent_application(
    conn: &mut AsyncPgConnection,
    requested: &NewTeacherApplication,
) -> Result<Option<TeacherApplication>, TeacherApplicationError> {
    let Some(idempotency_key) = requested.idempotency_key.as_deref() else {
        return Ok(None);
    };

    let existing =
        teacher_application_repository::find_application_by_idempotency_key(conn, idempotency_key)
            .await?;
    if let Some(existing) = existing.as_ref() {
        ensure_idempotent_application_matches(existing, requested)?;
    }

    Ok(existing)
}

async fn ensure_no_blocking_application(
    conn: &mut AsyncPgConnection,
    requested: &NewTeacherApplication,
) -> Result<(), TeacherApplicationError> {
    let existing = teacher_application_repository::find_latest_application_for_applicant(
        conn,
        requested.applicant_user_id,
    )
    .await?;

    match existing {
        Some(application) if application.status != TEACHER_APPLICATION_STATUS_REJECTED => {
            Err(TeacherApplicationError::InvalidTransition(format!(
                "teacher application already exists with status {}",
                application.status
            )))
        }
        _ => Ok(()),
    }
}

fn ensure_idempotent_application_matches(
    existing: &TeacherApplication,
    requested: &NewTeacherApplication,
) -> Result<(), TeacherApplicationError> {
    if existing.applicant_user_id == requested.applicant_user_id
        && existing.requested_scope == requested.requested_scope
        && existing.requested_organization_id == requested.requested_organization_id
        && existing.requested_course_id == requested.requested_course_id
        && existing.experience_summary == requested.experience_summary
        && existing.organization_sponsor_id == requested.organization_sponsor_id
        && existing.portfolio_links == requested.portfolio_links
    {
        Ok(())
    } else {
        Err(TeacherApplicationError::InvalidInput(
            "teacher application idempotency key is already used by another application"
                .to_string(),
        ))
    }
}

fn normalize_idempotency_key(
    idempotency_key: Option<String>,
) -> Result<Option<String>, TeacherApplicationError> {
    match idempotency_key {
        Some(value) => {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                Err(TeacherApplicationError::InvalidInput(
                    "idempotency_key cannot be blank".to_string(),
                ))
            } else {
                Ok(Some(trimmed))
            }
        }
        None => Ok(None),
    }
}

fn validate_requested_scope(
    requested_scope: &str,
    requested_organization_id: Option<i32>,
    requested_course_id: Option<i32>,
    organization_sponsor_id: Option<i32>,
) -> Result<(), TeacherApplicationError> {
    match requested_scope {
        TEACHER_APPLICATION_SCOPE_PLATFORM => Ok(()),
        TEACHER_APPLICATION_SCOPE_ORGANIZATION => {
            if requested_organization_id
                .or(organization_sponsor_id)
                .is_none()
            {
                return Err(TeacherApplicationError::InvalidInput(
                    "organization scope requires a requested organization or sponsor".to_string(),
                ));
            }
            Ok(())
        }
        TEACHER_APPLICATION_SCOPE_COURSE => {
            if requested_course_id.is_none() {
                return Err(TeacherApplicationError::InvalidInput(
                    "course scope requires requested_course_id".to_string(),
                ));
            }
            Ok(())
        }
        _ => Err(TeacherApplicationError::InvalidInput(
            "unsupported requested_scope".to_string(),
        )),
    }
}

fn clean_portfolio_links(portfolio_links: Option<Vec<String>>) -> Vec<String> {
    portfolio_links
        .unwrap_or_default()
        .into_iter()
        .map(|link| link.trim().to_string())
        .filter(|link| !link.is_empty())
        .collect()
}

fn portfolio_links_from_json(portfolio_links: &serde_json::Value) -> Vec<String> {
    portfolio_links
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str())
                .map(|link| link.trim().to_string())
                .filter(|link| !link.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn normalize_scope(scope: &str) -> Result<String, TeacherApplicationError> {
    let normalized = scope.trim().to_ascii_lowercase();
    match normalized.as_str() {
        TEACHER_APPLICATION_SCOPE_PLATFORM
        | TEACHER_APPLICATION_SCOPE_ORGANIZATION
        | TEACHER_APPLICATION_SCOPE_COURSE => Ok(normalized),
        _ => Err(TeacherApplicationError::InvalidInput(
            "unsupported requested_scope".to_string(),
        )),
    }
}

fn normalize_status(status: &str) -> Result<String, TeacherApplicationError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        TEACHER_APPLICATION_STATUS_SUBMITTED
        | TEACHER_APPLICATION_STATUS_NEEDS_CHANGES
        | TEACHER_APPLICATION_STATUS_APPROVED
        | TEACHER_APPLICATION_STATUS_REJECTED => Ok(normalized),
        _ => Err(TeacherApplicationError::InvalidInput(
            "unsupported teacher application status".to_string(),
        )),
    }
}

fn normalize_decision_status(status: &str) -> Result<String, TeacherApplicationError> {
    let normalized = normalize_status(status)?;
    match normalized.as_str() {
        TEACHER_APPLICATION_STATUS_APPROVED
        | TEACHER_APPLICATION_STATUS_REJECTED
        | TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => Ok(normalized),
        _ => Err(TeacherApplicationError::InvalidInput(
            "decision status must be approved, rejected, or needs_changes".to_string(),
        )),
    }
}
