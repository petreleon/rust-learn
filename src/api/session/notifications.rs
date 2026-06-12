use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::notifications::get_preferences;
use crate::application::notifications::preferences::NotificationPreferencesError;
use crate::application::notifications::save_preferences;
use crate::db;
use crate::http::notifications::dto::{
    NotificationPreferencesRequest, NotificationPreferencesResponse,
};
use crate::infra::postgres::notifications::notification_preference_store::PostgresNotificationPreferenceStore;
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user_id;

pub async fn get_notification_preferences(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
) -> impl Responder {
    let user_id_val = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };

    let mut store = PostgresNotificationPreferenceStore::new(&mut conn);

    match get_preferences::get_preferences(&mut store, user_id_val).await {
        Ok(preferences) => {
            HttpResponse::Ok().json(NotificationPreferencesResponse::from(preferences))
        }
        Err(e) => {
            log::error!(
                "event=prefs_fetch_failed user_id={} error={}",
                user_id_val,
                notification_preferences_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to load preferences")
        }
    }
}

pub async fn save_notification_preferences(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
    body: web::Json<NotificationPreferencesRequest>,
) -> impl Responder {
    let user_id_val = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };

    let mut store = PostgresNotificationPreferenceStore::new(&mut conn);
    let command = body.into_inner().into_command(user_id_val);

    match save_preferences::save_preferences(&mut store, command).await {
        Ok(preferences) => {
            HttpResponse::Ok().json(NotificationPreferencesResponse::from(preferences))
        }
        Err(e) => {
            log::error!(
                "event=prefs_save_failed user_id={} error={}",
                user_id_val,
                notification_preferences_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to save preferences")
        }
    }
}

pub async fn list_notifications(pool: web::Data<db::DbPool>, req: HttpRequest) -> impl Responder {
    let user_id_val = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let notifications = NotificationsState::from(pool.get_ref().clone());

    match notifications.get_notifications(user_id_val).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            log::error!(
                "event=notifications_list_failed user_id={} error={}",
                user_id_val,
                e
            );
            HttpResponse::InternalServerError().body("Failed to load notifications")
        }
    }
}

pub async fn mark_notification_read(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> impl Responder {
    let user_id_val = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let notification_id = path.into_inner();

    let notifications = NotificationsState::from(pool.get_ref().clone());

    match notifications.mark_read(user_id_val, notification_id).await {
        Ok(()) => HttpResponse::Ok().body("Notification marked as read"),
        Err(e) => {
            log::error!(
                "event=notification_mark_read_failed user_id={} notification_id={} error={}",
                user_id_val,
                notification_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to mark notification as read")
        }
    }
}

pub async fn clear_notifications(pool: web::Data<db::DbPool>, req: HttpRequest) -> impl Responder {
    let user_id_val = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let notifications = NotificationsState::from(pool.get_ref().clone());

    match notifications.clear(user_id_val).await {
        Ok(()) => HttpResponse::Ok().body("Notifications cleared"),
        Err(e) => {
            log::error!(
                "event=notifications_clear_failed user_id={} error={}",
                user_id_val,
                e
            );
            HttpResponse::InternalServerError().body("Failed to clear notifications")
        }
    }
}

fn notification_preferences_error_log(error: &NotificationPreferencesError) -> String {
    match error {
        NotificationPreferencesError::Database(message) => message.clone(),
    }
}
