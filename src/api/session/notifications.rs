use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user_id;

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
