use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::application::identity::email::{email_log_hash, normalize_email};
use crate::application::identity::resend_verification::{
    ResendVerificationCommand, ResendVerificationError, ResendVerificationOutcome,
    ResendVerificationUseCase,
};

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
) -> impl Responder {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return HttpResponse::BadRequest().body("Email is required");
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
            generic_success_response()
        }
        Ok(ResendVerificationOutcome::Sent | ResendVerificationOutcome::AlreadyVerified) => {
            generic_success_response()
        }
        Err(error) => resend_verification_error_response(error),
    }
}

fn generic_success_response() -> HttpResponse {
    HttpResponse::Ok().body(RESEND_VERIFICATION_MESSAGE)
}

fn resend_verification_error_response(error: ResendVerificationError) -> HttpResponse {
    match error {
        ResendVerificationError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        ResendVerificationError::Lookup(error) => {
            log::error!("event=email_verification_resend_lookup_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to resend verification email")
        }
        ResendVerificationError::TokenGeneration(error) => {
            log::error!("event=email_verification_resend_token_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to create email verification token")
        }
        ResendVerificationError::Store(error) => {
            log::error!("event=email_verification_resend_store_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to resend verification email")
        }
    }
}
