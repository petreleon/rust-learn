use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::verify;
use serde::Deserialize;

use super::support::{email_log_hash, normalize_email};
use crate::db;
use crate::models::user::User;
use crate::utils::jwt_utils::create_jwt;

#[derive(Deserialize)]
pub(super) struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
pub(super) async fn login(
    pool: web::Data<db::DbPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let email = normalize_email(&req.email);
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let user_auth_result = User::find_with_password_auth(&email, &mut conn).await;

    match user_auth_result {
        Ok((user, Some(hash))) if verify(&req.password, &hash).unwrap_or(false) => {
            if !user.email_verified {
                log::info!(
                    "event=auth_login_denied reason=email_unverified user_id={} email_hash={}",
                    user.id(),
                    email_log_hash(&email)
                );
                return HttpResponse::Forbidden().body("Email verification required");
            }

            match create_jwt(user.id()) {
                Ok(user_jwt) => HttpResponse::Ok().json(user_jwt),
                Err(err) => {
                    log::error!(
                        "event=auth_jwt_create_failed user_id={} error={}",
                        user.id(),
                        err
                    );
                    HttpResponse::InternalServerError().body("Failed to create JWT")
                }
            }
        }
        Ok((_, Some(_))) | Err(_) => {
            log::info!(
                "event=auth_login_failed reason=invalid_credentials email_hash={}",
                email_log_hash(&email)
            );
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
        Ok((_user, None)) => {
            log::warn!(
                "event=auth_login_failed reason=missing_password_auth email_hash={}",
                email_log_hash(&email)
            );
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
    }
}
