use std::sync::Arc;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::application::identity::verify_email::{
    VerifyEmailCommand, VerifyEmailError, VerifyEmailOutcome, VerifyEmailUseCase,
};

#[derive(Deserialize)]
pub(super) struct VerifyEmailQuery {
    token: String,
}

#[get("/verify-email")]
pub(super) async fn verify_email(
    use_case: web::Data<Arc<dyn VerifyEmailUseCase>>,
    query: web::Query<VerifyEmailQuery>,
) -> impl Responder {
    let token = query.token.trim();
    if token.is_empty() {
        return HttpResponse::BadRequest().body("Verification token is required");
    }

    match use_case
        .verify_email(VerifyEmailCommand {
            token: token.to_string(),
        })
        .await
    {
        Ok(VerifyEmailOutcome::Verified) => HttpResponse::Ok().body("Email verified successfully"),
        Ok(VerifyEmailOutcome::AlreadyVerified) => {
            HttpResponse::Ok().body("Email already verified")
        }
        Ok(VerifyEmailOutcome::Expired) => {
            HttpResponse::BadRequest().body("Verification token expired")
        }
        Ok(VerifyEmailOutcome::Invalid) => {
            HttpResponse::BadRequest().body("Invalid verification token")
        }
        Err(error) => {
            log::error!(
                "event=email_verification_failed error={}",
                verify_email_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Failed to verify email token")
        }
    }
}

fn verify_email_error_log(error: &VerifyEmailError) -> String {
    match error {
        VerifyEmailError::Connection(message) | VerifyEmailError::Database(message) => {
            message.clone()
        }
    }
}
