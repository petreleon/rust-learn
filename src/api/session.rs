use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

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
