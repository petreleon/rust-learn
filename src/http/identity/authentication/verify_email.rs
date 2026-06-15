use std::sync::Arc;

use actix_web::{get, web};
use serde::Deserialize;

use crate::application::identity::verify_email::{
    VerifyEmailCommand, VerifyEmailOutcome, VerifyEmailUseCase,
};
use crate::http::identity::authentication::errors::verify_email_error;
use crate::http::identity::authentication::text_error::AuthTextError;

#[derive(Deserialize)]
pub(super) struct VerifyEmailQuery {
    token: String,
}

#[get("/verify-email")]
pub(super) async fn verify_email(
    use_case: web::Data<Arc<dyn VerifyEmailUseCase>>,
    query: web::Query<VerifyEmailQuery>,
) -> Result<&'static str, AuthTextError> {
    let token = query.token.trim();
    if token.is_empty() {
        return Err(AuthTextError::bad_request("Verification token is required"));
    }

    match use_case
        .verify_email(VerifyEmailCommand {
            token: token.to_string(),
        })
        .await
    {
        Ok(VerifyEmailOutcome::Verified) => Ok("Email verified successfully"),
        Ok(VerifyEmailOutcome::AlreadyVerified) => Ok("Email already verified"),
        Ok(VerifyEmailOutcome::Expired) => {
            Err(AuthTextError::bad_request("Verification token expired"))
        }
        Ok(VerifyEmailOutcome::Invalid) => {
            Err(AuthTextError::bad_request("Invalid verification token"))
        }
        Err(error) => Err(verify_email_error(error)),
    }
}
