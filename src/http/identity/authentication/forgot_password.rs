use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use super::support::{email_log_hash, normalize_email};
use crate::application::identity::request_password_reset::{
    RequestPasswordResetCommand, RequestPasswordResetError, RequestPasswordResetOutcome,
    RequestPasswordResetUseCase,
};

const PASSWORD_RESET_REQUEST_MESSAGE: &str =
    "If an account matches that email, a password reset link has been sent.";

#[derive(Deserialize)]
pub(super) struct ForgotPasswordRequest {
    email: String,
}

#[post("/forgot-password")]
pub(super) async fn forgot_password(
    use_case: web::Data<Arc<dyn RequestPasswordResetUseCase>>,
    req: web::Json<ForgotPasswordRequest>,
) -> impl Responder {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return HttpResponse::BadRequest().body("Email is required");
    }

    match use_case
        .request_password_reset(RequestPasswordResetCommand {
            email: email.clone(),
        })
        .await
    {
        Ok(RequestPasswordResetOutcome::UnknownEmail) => {
            log::info!(
                "event=password_reset_requested_unknown_email email_hash={}",
                email_log_hash(&email)
            );
            generic_success_response()
        }
        Ok(RequestPasswordResetOutcome::Sent) => generic_success_response(),
        Err(error) => request_password_reset_error_response(error),
    }
}

fn generic_success_response() -> HttpResponse {
    HttpResponse::Ok().body(PASSWORD_RESET_REQUEST_MESSAGE)
}

fn request_password_reset_error_response(error: RequestPasswordResetError) -> HttpResponse {
    match error {
        RequestPasswordResetError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        RequestPasswordResetError::Lookup(error) => {
            log::error!("event=password_reset_lookup_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to request password reset")
        }
        RequestPasswordResetError::TokenGeneration(error) => {
            log::error!("event=password_reset_token_generate_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to create password reset token")
        }
        RequestPasswordResetError::Store(error) => {
            log::error!("event=password_reset_token_store_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to request password reset")
        }
    }
}
