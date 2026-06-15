use actix_web::http::StatusCode;

use crate::application::notifications::notification_inbox::NotificationInboxError;
use crate::application::notifications::preferences::NotificationPreferencesError;
use crate::http::errors::ApiError;

pub(super) fn notification_preferences_error(
    event: &'static str,
    user_id: i32,
    failure_message: &'static str,
    error: NotificationPreferencesError,
) -> ApiError {
    match error {
        NotificationPreferencesError::Connection(_) => ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "db_unavailable",
            "DB unavailable",
        ),
        NotificationPreferencesError::Database(message) => {
            log::error!("event={} user_id={} error={}", event, user_id, message);
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "notification_preferences_unavailable",
                failure_message,
            )
        }
    }
}

pub(super) fn notification_inbox_error(
    event: &'static str,
    user_id: i32,
    notification_id: Option<i64>,
    failure_message: &'static str,
    error: NotificationInboxError,
) -> ApiError {
    let message = notification_inbox_error_message(error);
    log_notification_inbox_error(event, user_id, notification_id, &message);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "notification_inbox_unavailable",
        failure_message,
    )
}

fn notification_inbox_error_message(error: NotificationInboxError) -> String {
    match error {
        NotificationInboxError::Connection(message) | NotificationInboxError::Database(message) => {
            message
        }
    }
}

fn log_notification_inbox_error(
    event: &'static str,
    user_id: i32,
    notification_id: Option<i64>,
    message: &str,
) {
    if let Some(notification_id) = notification_id {
        log::error!(
            "event={} user_id={} notification_id={} error={}",
            event,
            user_id,
            notification_id,
            message
        );
    } else {
        log::error!("event={} user_id={} error={}", event, user_id, message);
    }
}

#[cfg(test)]
mod tests {
    use super::{notification_inbox_error, notification_preferences_error};
    use crate::application::notifications::notification_inbox::NotificationInboxError;
    use crate::application::notifications::preferences::NotificationPreferencesError;
    use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
    use serde_json::Value;

    #[actix_web::test]
    async fn preferences_connection_error_uses_api_error_envelope() {
        let response = notification_preferences_error(
            "prefs_fetch_failed",
            42,
            "Failed to load preferences",
            NotificationPreferencesError::Connection("closed".to_string()),
        )
        .error_response();

        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body.value["error"]["code"], "db_unavailable");
        assert_eq!(body.value["error"]["message"], "DB unavailable");
        assert_eq!(body.value["error"]["status"], 500);
    }

    #[actix_web::test]
    async fn preferences_database_error_uses_context_error_code() {
        let response = notification_preferences_error(
            "prefs_save_failed",
            42,
            "Failed to save preferences",
            NotificationPreferencesError::Database("failed".to_string()),
        )
        .error_response();

        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            body.value["error"]["code"],
            "notification_preferences_unavailable"
        );
        assert_eq!(body.value["error"]["message"], "Failed to save preferences");
        assert_eq!(body.value["error"]["status"], 500);
    }

    #[actix_web::test]
    async fn inbox_error_uses_context_error_code() {
        let response = notification_inbox_error(
            "notification_mark_read_failed",
            42,
            Some(7),
            "Failed to mark notification as read",
            NotificationInboxError::Database("failed".to_string()),
        )
        .error_response();

        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            body.value["error"]["code"],
            "notification_inbox_unavailable"
        );
        assert_eq!(
            body.value["error"]["message"],
            "Failed to mark notification as read"
        );
        assert_eq!(body.value["error"]["status"], 500);
    }

    struct ParsedErrorBody {
        status: StatusCode,
        value: Value,
    }

    async fn parse_body(response: actix_web::HttpResponse) -> ParsedErrorBody {
        let status = response.status();
        let body = to_bytes(response.into_body()).await.unwrap();
        let value = serde_json::from_slice(&body).unwrap();
        ParsedErrorBody { status, value }
    }
}
