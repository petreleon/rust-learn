use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

use super::support::{email_log_hash, normalize_email};
use crate::db;
use crate::models::email_verification_token::EmailVerificationToken;
use crate::models::user::User;
use crate::utils::email::{
    generate_verification_token, print_mock_verification_email, verification_token_hash,
};

const RESEND_VERIFICATION_MESSAGE: &str =
    "If an unverified account matches that email, a verification link has been sent.";

#[derive(Deserialize)]
pub(super) struct ResendVerificationRequest {
    email: String,
}

#[post("/resend-verification")]
pub(super) async fn resend_verification(
    pool: web::Data<db::DbPool>,
    req: web::Json<ResendVerificationRequest>,
) -> impl Responder {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return HttpResponse::BadRequest().body("Email is required");
    }

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let user = match User::find_by_email(&email, &mut conn).await.optional() {
        Ok(user) => user,
        Err(err) => {
            log::error!("event=email_verification_resend_lookup_failed error={err}");
            return HttpResponse::InternalServerError().body("Failed to resend verification email");
        }
    };

    if let Some(user) = user {
        if !user.email_verified {
            let verification_token = match generate_verification_token() {
                Ok(token) => token,
                Err(err) => {
                    log::error!("event=email_verification_resend_token_failed error={err}");
                    return HttpResponse::InternalServerError()
                        .body("Failed to create email verification token");
                }
            };
            if let Err(err) = EmailVerificationToken::create_for_user(
                &mut conn,
                user.id(),
                verification_token_hash(&verification_token),
            )
            .await
            {
                log::error!("event=email_verification_resend_store_failed error={err}");
                return HttpResponse::InternalServerError()
                    .body("Failed to resend verification email");
            }
            print_mock_verification_email(&user.email, &user.name, &verification_token);
        }
    } else {
        log::info!(
            "event=email_verification_resend_unknown_email email_hash={}",
            email_log_hash(&email)
        );
    }

    HttpResponse::Ok().body(RESEND_VERIFICATION_MESSAGE)
}
