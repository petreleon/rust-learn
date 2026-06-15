use std::sync::Arc;

use actix_web::{post, web};
use serde::Deserialize;

use crate::application::identity::password_policy::validate_password_strength;
use crate::application::identity::reset_password::{
    ResetPasswordCommand, ResetPasswordOutcome, ResetPasswordUseCase,
};
use crate::http::identity::authentication::errors::reset_password_error;
use crate::http::identity::authentication::text_error::AuthTextError;

#[derive(Deserialize)]
pub(super) struct ResetPasswordRequest {
    token: String,
    password: String,
}

#[post("/reset-password")]
pub(super) async fn reset_password(
    use_case: web::Data<Arc<dyn ResetPasswordUseCase>>,
    req: web::Json<ResetPasswordRequest>,
) -> Result<&'static str, AuthTextError> {
    let token = req.token.trim();
    if token.is_empty() {
        return Err(AuthTextError::bad_request(
            "Password reset token is required",
        ));
    }
    if let Err(message) = validate_password_strength(&req.password) {
        return Err(AuthTextError::bad_request(message));
    }

    match use_case
        .reset_password(ResetPasswordCommand {
            token: token.to_string(),
            password: req.password.to_string(),
        })
        .await
    {
        Ok(ResetPasswordOutcome::Reset) => Ok("Password updated successfully"),
        Ok(ResetPasswordOutcome::Expired) => {
            Err(AuthTextError::bad_request("Password reset token expired"))
        }
        Ok(ResetPasswordOutcome::Invalid) => {
            Err(AuthTextError::bad_request("Invalid password reset token"))
        }
        Err(error) => Err(reset_password_error(error)),
    }
}
