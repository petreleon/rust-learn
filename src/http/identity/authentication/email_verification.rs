use std::sync::Arc;

use actix_web::{post, web};
use serde::Deserialize;

use crate::application::identity::email::{email_log_hash, normalize_email};
use crate::application::identity::resend_verification::{
    ResendVerificationCommand, ResendVerificationOutcome, ResendVerificationUseCase,
};
use crate::http::identity::authentication::errors::{
    missing_email_error, resend_verification_error,
};
use crate::http::identity::authentication::text_error::AuthTextError;

const RESEND_VERIFICATION_MESSAGE: &str =
    "If an unverified account matches that email, a verification link has been sent.";

#[derive(Deserialize)]
pub(super) struct ResendVerificationRequest {
    email: String,
}

#[post("/resend-verification")]
pub(super) async fn resend_verification(
    use_case: web::Data<Arc<dyn ResendVerificationUseCase>>,
    req: web::Json<ResendVerificationRequest>,
) -> Result<&'static str, AuthTextError> {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return Err(missing_email_error());
    }

    match use_case
        .resend_verification(ResendVerificationCommand {
            email: email.clone(),
        })
        .await
    {
        Ok(ResendVerificationOutcome::UnknownEmail) => {
            log::info!(
                "event=email_verification_resend_unknown_email email_hash={}",
                email_log_hash(&email)
            );
            Ok(RESEND_VERIFICATION_MESSAGE)
        }
        Ok(ResendVerificationOutcome::Sent | ResendVerificationOutcome::AlreadyVerified) => {
            Ok(RESEND_VERIFICATION_MESSAGE)
        }
        Err(error) => Err(resend_verification_error(error)),
    }
}
