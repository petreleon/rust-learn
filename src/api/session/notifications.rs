use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::Deserialize;

use crate::db;
use crate::models::notification_preferences::{
    NotificationPreferences, UpsertNotificationPreferences,
};
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user_id;

#[derive(Deserialize)]
pub struct NotificationPreferencesPayload {
    email_enabled: bool,
    push_enabled: bool,
}

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

    let prefs = crate::db::schema::user_notification_preferences::table
        .filter(crate::db::schema::user_notification_preferences::user_id.eq(user_id_val))
        .first::<NotificationPreferences>(&mut conn)
        .await;

    match prefs {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(diesel::result::Error::NotFound) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": user_id_val,
            "email_enabled": true,
            "push_enabled": false,
        })),
        Err(e) => {
            log::error!(
                "event=prefs_fetch_failed user_id={} error={}",
                user_id_val,
                e
            );
            HttpResponse::InternalServerError().body("Failed to load preferences")
        }
    }
}

pub async fn save_notification_preferences(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
    body: web::Json<NotificationPreferencesPayload>,
) -> impl Responder {
    let user_id_val = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };

    diesel::delete(
        crate::db::schema::user_notification_preferences::table
            .filter(crate::db::schema::user_notification_preferences::user_id.eq(user_id_val)),
    )
    .execute(&mut conn)
    .await
    .ok();

    let payload = UpsertNotificationPreferences {
        user_id: user_id_val,
        email_enabled: body.email_enabled,
        push_enabled: body.push_enabled,
    };

    let result: Result<NotificationPreferences, _> =
        diesel::insert_into(crate::db::schema::user_notification_preferences::table)
            .values(&payload)
            .get_result(&mut conn)
            .await;

    match result {
        Ok(prefs) => HttpResponse::Ok().json(prefs),
        Err(e) => {
            log::error!(
                "event=prefs_save_failed user_id={} error={}",
                user_id_val,
                e
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
