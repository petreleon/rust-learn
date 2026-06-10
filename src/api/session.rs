use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::services::session_service::{self, CurrentSessionError};
use crate::utils::request_auth::authenticated_user_id;

#[derive(Serialize)]
struct SessionErrorEnvelope {
    error: SessionErrorBody,
}

#[derive(Serialize)]
struct SessionErrorBody {
    code: &'static str,
    message: &'static str,
}

fn session_error(code: &'static str, message: &'static str) -> SessionErrorEnvelope {
    SessionErrorEnvelope {
        error: SessionErrorBody { code, message },
    }
}

pub async fn get_current_session(pool: web::Data<db::DbPool>, req: HttpRequest) -> impl Responder {
    let current_user_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => {
            return HttpResponse::build(response.status()).json(session_error(
                "unauthorized",
                "A valid bearer token is required.",
            ));
        }
    };

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(error) => {
            log::error!("event=session_current_user_db_pool_failed error={}", error);
            return HttpResponse::InternalServerError().json(session_error(
                "database_unavailable",
                "Current session could not be loaded.",
            ));
        }
    };

    match session_service::current_session(&mut conn, current_user_id).await {
        Ok(session) => HttpResponse::Ok().json(session),
        Err(CurrentSessionError::MissingUser) => HttpResponse::NotFound().json(session_error(
            "missing_user",
            "The authenticated user no longer exists.",
        )),
        Err(CurrentSessionError::EmailUnverified) => HttpResponse::Forbidden().json(session_error(
            "unverified_email",
            "Email verification is required before using this session.",
        )),
        Err(CurrentSessionError::Database(error)) => {
            log::error!(
                "event=session_current_user_load_failed user_id={} error={}",
                current_user_id,
                error
            );
            HttpResponse::InternalServerError().json(session_error(
                "session_load_failed",
                "Current session could not be loaded.",
            ))
        }
    }
}

#[derive(Deserialize)]
pub struct NotificationPreferencesPayload {
    email_enabled: bool,
    push_enabled: bool,
}

pub async fn get_notification_preferences(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
) -> impl Responder {
    use crate::models::notification_preferences::NotificationPreferences;

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
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::Ok().json(serde_json::json!({
                "user_id": user_id_val,
                "email_enabled": true,
                "push_enabled": false,
            }))
        }
        Err(e) => {
            log::error!("event=prefs_fetch_failed user_id={} error={}", user_id_val, e);
            HttpResponse::InternalServerError().body("Failed to load preferences")
        }
    }
}

pub async fn save_notification_preferences(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
    body: web::Json<NotificationPreferencesPayload>,
) -> impl Responder {
    use crate::models::notification_preferences::{
        NotificationPreferences, UpsertNotificationPreferences,
    };

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

    let result: Result<NotificationPreferences, _> = diesel::insert_into(
        crate::db::schema::user_notification_preferences::table,
    )
    .values(&payload)
    .get_result(&mut conn)
    .await;

    match result {
        Ok(prefs) => HttpResponse::Ok().json(prefs),
        Err(e) => {
            log::error!("event=prefs_save_failed user_id={} error={}", user_id_val, e);
            HttpResponse::InternalServerError().body("Failed to save preferences")
        }
    }
}

pub async fn list_notifications(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
) -> impl Responder {
    use crate::models::notification::Notification;

    let user_id_val = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };

    match Notification::find_by_user_id(user_id_val, &mut conn).await {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(e) => {
            log::error!("event=notifications_list_failed user_id={} error={}", user_id_val, e);
            HttpResponse::InternalServerError().body("Failed to load notifications")
        }
    }
}

pub async fn mark_notification_read(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> impl Responder {
    use crate::models::notification::Notification;

    let user_id_val = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let notification_id = path.into_inner();

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };

    match Notification::mark_as_read(user_id_val, notification_id, &mut conn).await {
        Ok(_) => HttpResponse::Ok().body("Notification marked as read"),
        Err(e) => {
            log::error!(
                "event=notification_mark_read_failed user_id={} notification_id={} error={}",
                user_id_val, notification_id, e
            );
            HttpResponse::InternalServerError().body("Failed to mark notification as read")
        }
    }
}
