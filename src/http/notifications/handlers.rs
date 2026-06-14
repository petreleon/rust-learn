use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::notifications::notification_inbox::{
    NotificationInboxError, NotificationInboxUseCase, NotificationOutput,
};
use crate::application::notifications::preference_service::NotificationPreferencesUseCase;
use crate::application::notifications::preferences::NotificationPreferencesError;
use crate::http::extractors::request_auth::authenticated_user_id;
use crate::http::notifications::dto::{
    NotificationPreferencesRequest, NotificationPreferencesResponse, NotificationResponse,
};

pub async fn get_notification_preferences(
    preferences: web::Data<Arc<dyn NotificationPreferencesUseCase>>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match preferences.get_preferences(user_id).await {
        Ok(preferences) => {
            HttpResponse::Ok().json(NotificationPreferencesResponse::from(preferences))
        }
        Err(NotificationPreferencesError::Connection(_)) => {
            HttpResponse::InternalServerError().body("DB unavailable")
        }
        Err(error) => {
            log::error!(
                "event=prefs_fetch_failed user_id={} error={}",
                user_id,
                notification_preferences_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Failed to load preferences")
        }
    }
}

pub async fn save_notification_preferences(
    preferences: web::Data<Arc<dyn NotificationPreferencesUseCase>>,
    req: HttpRequest,
    body: web::Json<NotificationPreferencesRequest>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let command = body.into_inner().into_command(user_id);

    match preferences.save_preferences(command).await {
        Ok(preferences) => {
            HttpResponse::Ok().json(NotificationPreferencesResponse::from(preferences))
        }
        Err(NotificationPreferencesError::Connection(_)) => {
            HttpResponse::InternalServerError().body("DB unavailable")
        }
        Err(error) => {
            log::error!(
                "event=prefs_save_failed user_id={} error={}",
                user_id,
                notification_preferences_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Failed to save preferences")
        }
    }
}

pub async fn list_notifications(
    inbox: web::Data<Arc<dyn NotificationInboxUseCase>>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match inbox.list_notifications(user_id).await {
        Ok(list) => HttpResponse::Ok().json(notification_responses(list)),
        Err(error) => {
            log::error!(
                "event=notifications_list_failed user_id={} error={}",
                user_id,
                notification_inbox_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Failed to load notifications")
        }
    }
}

pub async fn mark_notification_read(
    inbox: web::Data<Arc<dyn NotificationInboxUseCase>>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let notification_id = path.into_inner();

    match inbox.mark_notification_read(user_id, notification_id).await {
        Ok(()) => HttpResponse::Ok().body("Notification marked as read"),
        Err(error) => {
            log::error!(
                "event=notification_mark_read_failed user_id={} notification_id={} error={}",
                user_id,
                notification_id,
                notification_inbox_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Failed to mark notification as read")
        }
    }
}

pub async fn clear_notifications(
    inbox: web::Data<Arc<dyn NotificationInboxUseCase>>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match inbox.clear_notifications(user_id).await {
        Ok(()) => HttpResponse::Ok().body("Notifications cleared"),
        Err(error) => {
            log::error!(
                "event=notifications_clear_failed user_id={} error={}",
                user_id,
                notification_inbox_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Failed to clear notifications")
        }
    }
}

fn notification_preferences_error_log(error: &NotificationPreferencesError) -> String {
    match error {
        NotificationPreferencesError::Connection(message)
        | NotificationPreferencesError::Database(message) => message.clone(),
    }
}

fn notification_responses(list: Vec<NotificationOutput>) -> Vec<NotificationResponse> {
    list.into_iter().map(NotificationResponse::from).collect()
}

fn notification_inbox_error_log(error: &NotificationInboxError) -> String {
    match error {
        NotificationInboxError::Connection(message) | NotificationInboxError::Database(message) => {
            message.clone()
        }
    }
}
