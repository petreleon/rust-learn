use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::notifications::preference_service::NotificationPreferencesUseCase;
use crate::application::notifications::preferences::NotificationPreferencesError;
use crate::http::notifications::dto::{
    NotificationPreferencesRequest, NotificationPreferencesResponse,
};
use crate::utils::request_auth::authenticated_user_id;

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

fn notification_preferences_error_log(error: &NotificationPreferencesError) -> String {
    match error {
        NotificationPreferencesError::Connection(message)
        | NotificationPreferencesError::Database(message) => message.clone(),
    }
}
