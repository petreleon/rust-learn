use std::sync::Arc;

use actix_web::{post, web};
use serde::Deserialize;

use crate::application::identity::email::{email_log_hash, normalize_email};
use crate::application::identity::request_password_reset::{
    RequestPasswordResetCommand, RequestPasswordResetOutcome, RequestPasswordResetUseCase,
};
use crate::http::identity::authentication::errors::{
    missing_email_error, request_password_reset_error,
};
use crate::http::identity::authentication::text_error::AuthTextError;

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
) -> Result<&'static str, AuthTextError> {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return Err(missing_email_error());
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
            Ok(PASSWORD_RESET_REQUEST_MESSAGE)
        }
        Ok(RequestPasswordResetOutcome::Sent) => Ok(PASSWORD_RESET_REQUEST_MESSAGE),
        Err(error) => Err(request_password_reset_error(error)),
    }
}
