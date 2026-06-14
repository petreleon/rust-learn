use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use super::support::{email_log_hash, normalize_email};
use crate::application::identity::login::{LoginCommand, LoginError, LoginUseCase};

#[derive(Deserialize)]
pub(super) struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
pub(super) async fn login(
    use_case: web::Data<Arc<dyn LoginUseCase>>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let email = normalize_email(&req.email);

    match use_case
        .login(LoginCommand {
            email: email.clone(),
            password: req.password.clone(),
        })
        .await
    {
        Ok(output) => HttpResponse::Ok().json(output.jwt),
        Err(LoginError::EmailUnverified { user_id }) => {
            log::info!(
                "event=auth_login_denied reason=email_unverified user_id={} email_hash={}",
                user_id,
                email_log_hash(&email)
            );
            HttpResponse::Forbidden().body("Email verification required")
        }
        Err(LoginError::InvalidCredentials) => {
            log::info!(
                "event=auth_login_failed reason=invalid_credentials email_hash={}",
                email_log_hash(&email)
            );
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
        Err(LoginError::MissingPasswordAuthentication) => {
            log::warn!(
                "event=auth_login_failed reason=missing_password_auth email_hash={}",
                email_log_hash(&email)
            );
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
        Err(LoginError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(LoginError::Token { user_id, message }) => {
            log::error!(
                "event=auth_jwt_create_failed user_id={} error={}",
                user_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to create JWT")
        }
    }
}
