use crate::config::constants::permissions::Permissions;
use crate::config::constants::roles::Roles;
use crate::db::schema::{user_role_course, user_role_organization, user_role_platform};
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
