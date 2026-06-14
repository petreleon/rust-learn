use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::application::identity::current_session::{CurrentSessionError, CurrentSessionUseCase};
use crate::http::extractors::request_auth::authenticated_user_id;
use crate::http::identity::dto::CurrentSessionResponse;

#[derive(Serialize)]
struct SessionErrorEnvelope {
    error: SessionErrorBody,
}

#[derive(Serialize)]
struct SessionErrorBody {
    code: &'static str,
    message: &'static str,
}

pub async fn get_current_session(
    session: web::Data<Arc<dyn CurrentSessionUseCase>>,
    req: HttpRequest,
) -> impl Responder {
    let current_user_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => {
            return HttpResponse::build(response.status()).json(session_error(
                "unauthorized",
                "A valid bearer token is required.",
            ));
        }
    };

    match session.get_current_session(current_user_id).await {
        Ok(session) => HttpResponse::Ok().json(CurrentSessionResponse::from(session)),
        Err(CurrentSessionError::MissingUser) => HttpResponse::NotFound().json(session_error(
            "missing_user",
            "The authenticated user no longer exists.",
        )),
        Err(CurrentSessionError::EmailUnverified) => HttpResponse::Forbidden().json(session_error(
            "unverified_email",
            "Email verification is required before using this session.",
        )),
        Err(CurrentSessionError::Connection(error)) => {
            log::error!("event=session_current_user_db_pool_failed error={}", error);
            HttpResponse::InternalServerError().json(session_error(
                "database_unavailable",
                "Current session could not be loaded.",
            ))
        }
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

fn session_error(code: &'static str, message: &'static str) -> SessionErrorEnvelope {
    SessionErrorEnvelope {
        error: SessionErrorBody { code, message },
    }
}
