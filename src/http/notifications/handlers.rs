use std::sync::Arc;

use actix_web::web;

use crate::application::notifications::notification_inbox::{
    NotificationInboxUseCase, NotificationOutput,
};
use crate::application::notifications::preference_service::NotificationPreferencesUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::notifications::dto::{
    NotificationPreferencesRequest, NotificationPreferencesResponse, NotificationResponse,
};
use crate::http::notifications::errors::{
    notification_inbox_error, notification_preferences_error,
};

pub async fn get_notification_preferences(
    preferences: web::Data<Arc<dyn NotificationPreferencesUseCase>>,
    user: AuthUserId,
) -> Result<web::Json<NotificationPreferencesResponse>, ApiError> {
    let user_id = user.into_inner();

    preferences
        .get_preferences(user_id)
        .await
        .map(NotificationPreferencesResponse::from)
        .map(web::Json)
        .map_err(|error| {
            notification_preferences_error(
                "prefs_fetch_failed",
                user_id,
                "Failed to load preferences",
                error,
            )
        })
}

pub async fn save_notification_preferences(
    preferences: web::Data<Arc<dyn NotificationPreferencesUseCase>>,
    user: AuthUserId,
    body: web::Json<NotificationPreferencesRequest>,
) -> Result<web::Json<NotificationPreferencesResponse>, ApiError> {
    let user_id = user.into_inner();
    let command = body.into_inner().into_command(user_id);

    preferences
        .save_preferences(command)
        .await
        .map(NotificationPreferencesResponse::from)
        .map(web::Json)
        .map_err(|error| {
            notification_preferences_error(
                "prefs_save_failed",
                user_id,
                "Failed to save preferences",
                error,
            )
        })
}

pub async fn list_notifications(
    inbox: web::Data<Arc<dyn NotificationInboxUseCase>>,
    user: AuthUserId,
) -> Result<web::Json<Vec<NotificationResponse>>, ApiError> {
    let user_id = user.into_inner();

    inbox
        .list_notifications(user_id)
        .await
        .map(notification_responses)
        .map(web::Json)
        .map_err(|error| {
            notification_inbox_error(
                "notifications_list_failed",
                user_id,
                None,
                "Failed to load notifications",
                error,
            )
        })
}

pub async fn mark_notification_read(
    inbox: web::Data<Arc<dyn NotificationInboxUseCase>>,
    user: AuthUserId,
    path: web::Path<i64>,
) -> Result<&'static str, ApiError> {
    let user_id = user.into_inner();
    let notification_id = path.into_inner();

    inbox
        .mark_notification_read(user_id, notification_id)
        .await
        .map(|()| "Notification marked as read")
        .map_err(|error| {
            notification_inbox_error(
                "notification_mark_read_failed",
                user_id,
                Some(notification_id),
                "Failed to mark notification as read",
                error,
            )
        })
}

pub async fn clear_notifications(
    inbox: web::Data<Arc<dyn NotificationInboxUseCase>>,
    user: AuthUserId,
) -> Result<&'static str, ApiError> {
    let user_id = user.into_inner();

    inbox
        .clear_notifications(user_id)
        .await
        .map(|()| "Notifications cleared")
        .map_err(|error| {
            notification_inbox_error(
                "notifications_clear_failed",
                user_id,
                None,
                "Failed to clear notifications",
                error,
            )
        })
}

fn notification_responses(list: Vec<NotificationOutput>) -> Vec<NotificationResponse> {
    list.into_iter().map(NotificationResponse::from).collect()
}
