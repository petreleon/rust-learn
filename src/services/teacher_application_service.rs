use crate::config::constants::permissions::Permissions;
use crate::models::teacher_application::{
    NewTeacherApplication, NewTeacherApplicationAuditEvent, TeacherApplication,
    TeacherApplicationAuditEvent, TEACHER_APPLICATION_SCOPE_COURSE,
    TEACHER_APPLICATION_SCOPE_ORGANIZATION, TEACHER_APPLICATION_SCOPE_PLATFORM,
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::teacher_application_repository::{self, TeacherApplicationFilter};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use serde::Deserialize;
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrganizationTeacherNominationRequest {
    pub applicant_user_id: i32,
    pub requested_scope: Option<String>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub portfolio_links: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TeacherApplicationDecisionRequest {
    pub status: String,
    pub decision_reason: Option<String>,
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

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let application =
                teacher_application_repository::create_application(conn, new_application).await?;
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
    .map_err(TeacherApplicationError::from)
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
    };
    let new_application = build_new_application(
        request.applicant_user_id,
        submit_request,
        Some(organization_id),
    )?;

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let application =
                teacher_application_repository::create_application(conn, new_application).await?;
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
    .map_err(TeacherApplicationError::from)
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

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
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
        diesel::result::Error::RollbackTransaction => TeacherApplicationError::InvalidTransition(
            "final teacher applications cannot be changed".to_string(),
        ),
        other => TeacherApplicationError::from(other),
    })
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
    })
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
