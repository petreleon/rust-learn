use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::application::identity::password_policy::validate_password_strength;
use crate::application::identity::reset_password::{
    ResetPasswordCommand, ResetPasswordError, ResetPasswordOutcome, ResetPasswordUseCase,
};

#[derive(Deserialize)]
pub(super) struct ResetPasswordRequest {
    token: String,
    password: String,
}

#[post("/reset-password")]
pub(super) async fn reset_password(
    use_case: web::Data<Arc<dyn ResetPasswordUseCase>>,
    req: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    let token = req.token.trim();
    if token.is_empty() {
        return HttpResponse::BadRequest().body("Password reset token is required");
    }
    if let Err(message) = validate_password_strength(&req.password) {
        return HttpResponse::BadRequest().body(message);
    }

    match use_case
        .reset_password(ResetPasswordCommand {
            token: token.to_string(),
            password: req.password.to_string(),
        })
        .await
    {
        Ok(ResetPasswordOutcome::Reset) => HttpResponse::Ok().body("Password updated successfully"),
        Ok(ResetPasswordOutcome::Expired) => {
            HttpResponse::BadRequest().body("Password reset token expired")
        }
        Ok(ResetPasswordOutcome::Invalid) => {
            HttpResponse::BadRequest().body("Invalid password reset token")
        }
        Err(error) => reset_password_error_response(error),
    }
}

fn reset_password_error_response(error: ResetPasswordError) -> HttpResponse {
    match error {
        ResetPasswordError::MissingToken => {
            HttpResponse::BadRequest().body("Password reset token is required")
        }
        ResetPasswordError::InvalidPassword(message) => HttpResponse::BadRequest().body(message),
        ResetPasswordError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        ResetPasswordError::PasswordHash(error) => {
            log::error!("event=password_reset_hash_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to reset password")
        }
        ResetPasswordError::Store(error) => {
            log::error!("event=password_reset_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to reset password")
        }
    }
}
