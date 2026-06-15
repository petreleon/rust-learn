use actix_web::http::StatusCode;

use crate::application::organizations::manage_organizations::OrganizationManagementError;
use crate::http::errors::ApiError;

mod members;
mod read_models;

pub(super) use members::{
    organization_member_audit_error, organization_member_invite_error,
    organization_member_list_error, organization_member_removal_error,
    organization_member_role_assignment_error,
};
pub(super) use read_models::{
    organization_course_list_error, organization_dashboard_error,
    organization_teacher_application_list_error,
};

pub(super) fn organization_management_error(
    error: OrganizationManagementError,
    organization_id: Option<i32>,
    event: &'static str,
    message: &'static str,
) -> ApiError {
    match error {
        OrganizationManagementError::NotFound => organization_not_found(),
        OrganizationManagementError::Connection(error) => {
            db_connection_failed(event, management_context(organization_id), error)
        }
        OrganizationManagementError::Database(error) => {
            logged_internal(event, &management_context(organization_id), message, error)
        }
    }
}

pub(super) fn organization_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "organization_not_found",
        "Organization not found",
    )
}

pub(super) fn user_not_found(message: &'static str) -> ApiError {
    ApiError::new(StatusCode::NOT_FOUND, "user_not_found", message)
}

pub(super) fn permission_denied(message: &'static str) -> ApiError {
    ApiError::new(StatusCode::FORBIDDEN, "permission_denied", message)
}

pub(super) fn hierarchy_denied(message: &'static str) -> ApiError {
    ApiError::new(StatusCode::FORBIDDEN, "hierarchy_denied", message)
}

pub(super) fn invalid_input(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::BAD_REQUEST, "invalid_input", message)
}

pub(super) fn db_connection_failed(
    event: &'static str,
    context: String,
    error: String,
) -> ApiError {
    log::error!("event={} {} error={}", event, context, error);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "db_connection_failed",
        "Failed to get DB connection",
    )
}

pub(super) fn logged_internal(
    event: &'static str,
    context: &str,
    message: &'static str,
    error: String,
) -> ApiError {
    log::error!("event={} {} error={}", event, context, error);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "organization_request_failed",
        message,
    )
}

fn management_context(organization_id: Option<i32>) -> String {
    organization_id
        .map(|id| format!("organization_id={id}"))
        .unwrap_or_else(|| "organization_id=none".to_string())
}

#[cfg(test)]
mod tests;
