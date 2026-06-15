use actix_web::http::StatusCode;

use crate::application::identity::assign_platform_role::AssignPlatformRoleError;
use crate::application::identity::current_session::CurrentSessionError;
use crate::application::identity::user_profile::UserProfileError;
use crate::http::errors::ApiError;

const CURRENT_SESSION_LOAD_FAILED: &str = "Current session could not be loaded.";
const REQUIRED_PERMISSION: &str = "User does not have the required permission";

pub(super) fn current_session_error(user_id: i32, error: CurrentSessionError) -> ApiError {
    match error {
        CurrentSessionError::MissingUser => ApiError::new(
            StatusCode::NOT_FOUND,
            "missing_user",
            "The authenticated user no longer exists.",
        ),
        CurrentSessionError::EmailUnverified => ApiError::new(
            StatusCode::FORBIDDEN,
            "unverified_email",
            "Email verification is required before using this session.",
        ),
        CurrentSessionError::Connection(message) => {
            log::error!(
                "event=session_current_user_db_pool_failed error={}",
                message
            );
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_unavailable",
                CURRENT_SESSION_LOAD_FAILED,
            )
        }
        CurrentSessionError::Database(message) => {
            log::error!(
                "event=session_current_user_load_failed user_id={} error={}",
                user_id,
                message
            );
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "session_load_failed",
                CURRENT_SESSION_LOAD_FAILED,
            )
        }
    }
}

pub(super) fn current_session_auth_error() -> ApiError {
    ApiError::new(
        StatusCode::UNAUTHORIZED,
        "unauthorized",
        "A valid bearer token is required.",
    )
}

pub(super) fn get_user_profile_error(target_user_id: i32, error: UserProfileError) -> ApiError {
    match error {
        UserProfileError::Forbidden => ApiError::new(
            StatusCode::FORBIDDEN,
            "permission_denied",
            REQUIRED_PERMISSION,
        ),
        UserProfileError::NotFound => {
            ApiError::new(StatusCode::NOT_FOUND, "user_not_found", "User not found")
        }
        UserProfileError::Database(message) => {
            log::error!(
                "event=user_fetch_failed user_id={} error={}",
                target_user_id,
                message
            );
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "user_profile_unavailable",
                "Failed to fetch user",
            )
        }
    }
}

pub(super) fn list_users_error(error: UserProfileError) -> ApiError {
    log::error!(
        "event=user_list_failed error={}",
        user_profile_error_log(&error)
    );
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "users_unavailable",
        "Failed to load users",
    )
}

pub(super) fn platform_role_assignment_error(
    requester_id: i32,
    target_user_id: i32,
    role_name: &str,
    error: AssignPlatformRoleError,
) -> ApiError {
    match error {
        AssignPlatformRoleError::HierarchyViolation => ApiError::new(
            StatusCode::FORBIDDEN,
            "hierarchy_violation",
            "Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank.",
        ),
        AssignPlatformRoleError::RoleNotFound => ApiError::new(
            StatusCode::BAD_REQUEST,
            "role_not_found",
            format!("Role '{}' not found", role_name),
        ),
        AssignPlatformRoleError::Connection(message)
        | AssignPlatformRoleError::Database(message) => {
            log::error!(
                "event=platform_role_assign_failed requester_user_id={} target_user_id={} role={} error={}",
                requester_id,
                target_user_id,
                role_name,
                message
            );
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "platform_role_assignment_failed",
                "Failed to assign role",
            )
        }
    }
}

fn user_profile_error_log(error: &UserProfileError) -> String {
    match error {
        UserProfileError::Forbidden => "forbidden".to_string(),
        UserProfileError::NotFound => "not_found".to_string(),
        UserProfileError::Database(message) => message.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::{get_user_profile_error, platform_role_assignment_error};
    use crate::application::identity::assign_platform_role::AssignPlatformRoleError;
    use crate::application::identity::user_profile::UserProfileError;
    use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
    use serde_json::Value;

    #[actix_web::test]
    async fn user_profile_forbidden_uses_api_error_envelope() {
        let (status, value) =
            parse_body(get_user_profile_error(7, UserProfileError::Forbidden).error_response())
                .await;

        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(value["error"]["code"], "permission_denied");
        assert_eq!(value["error"]["message"], super::REQUIRED_PERMISSION);
    }

    #[actix_web::test]
    async fn role_not_found_uses_api_error_envelope() {
        let (status, value) = parse_body(
            platform_role_assignment_error(1, 2, "NOPE", AssignPlatformRoleError::RoleNotFound)
                .error_response(),
        )
        .await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(value["error"]["code"], "role_not_found");
        assert_eq!(value["error"]["message"], "Role 'NOPE' not found");
    }

    async fn parse_body(response: actix_web::HttpResponse) -> (StatusCode, Value) {
        let status = response.status();
        let body = to_bytes(response.into_body()).await.unwrap();
        let value = serde_json::from_slice(&body).unwrap();
        (status, value)
    }
}
