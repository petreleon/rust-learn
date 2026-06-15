use std::sync::Arc;

use actix_web::dev::Payload;
use actix_web::{http::StatusCode, web, Error, FromRequest, HttpMessage, HttpRequest};
use actix_web::{HttpResponse, Responder, ResponseError};
use futures::future::{ready, Ready};
use serde::Serialize;
use std::fmt;

use crate::application::identity::current_session::{CurrentSessionError, CurrentSessionUseCase};
use crate::domain::identity::UserJWT;
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
    user: CurrentSessionUserId,
) -> impl Responder {
    let current_user_id = user.into_inner();

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

pub struct CurrentSessionUserId(i32);

impl CurrentSessionUserId {
    fn into_inner(self) -> i32 {
        self.0
    }
}

impl FromRequest for CurrentSessionUserId {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(current_session_user_id(req).map_err(Into::into))
    }
}

fn current_session_user_id(
    req: &HttpRequest,
) -> Result<CurrentSessionUserId, CurrentSessionAuthError> {
    req.extensions()
        .get::<UserJWT>()
        .map(|user| CurrentSessionUserId(user.user_id))
        .ok_or(CurrentSessionAuthError)
}

#[derive(Debug)]
struct CurrentSessionAuthError;

impl fmt::Display for CurrentSessionAuthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("A valid bearer token is required.")
    }
}

impl ResponseError for CurrentSessionAuthError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(session_error(
            "unauthorized",
            "A valid bearer token is required.",
        ))
    }
}
