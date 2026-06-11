// src/api/authentication.rs
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bcrypt::{non_truncating_hash, verify, DEFAULT_COST};
use chrono::NaiveDate;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::AsyncConnection;
use serde::Deserialize;

use crate::db;
use crate::models::authentication::Authentication;
use crate::models::email_verification_token::{EmailVerificationResult, EmailVerificationToken};
use crate::models::role::PlatformRole;
use crate::models::user::{NewUser, User};
use crate::models::user_role_platform::UserRolePlatform;
use crate::utils::email::{
    generate_verification_token, print_mock_verification_email, verification_token_hash,
};
use crate::utils::jwt_utils::{create_jwt, public_jwks_from_env};
use crate::utils::request_auth::authenticated_user_id;

const MIN_PASSWORD_LENGTH: usize = 12;
const MAX_BCRYPT_PASSWORD_BYTES: usize = 71;
const PASSWORD_TOO_LONG_MESSAGE: &str =
    "Password must be at most 71 UTF-8 bytes for bcrypt hashing";

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct VerifyEmailQuery {
    pub token: String,
}

pub(crate) fn validate_password_strength(password: &str) -> Result<(), &'static str> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err("Password must be at least 12 characters long");
    }

    if password.len() > MAX_BCRYPT_PASSWORD_BYTES {
        return Err(PASSWORD_TOO_LONG_MESSAGE);
    }

    let has_lowercase = password.chars().any(char::is_lowercase);
    let has_uppercase = password.chars().any(char::is_uppercase);
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    if !(has_lowercase && has_uppercase && has_digit && has_symbol) {
        return Err("Password must include lowercase, uppercase, numeric, and symbol characters");
    }

    Ok(())
}

fn email_log_hash(email: &str) -> String {
    verification_token_hash(&normalize_email(email))
        .chars()
        .take(16)
        .collect()
}

fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn registration_db_error_response(error: DieselError, email: &str) -> HttpResponse {
    match error {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            log::warn!(
                "event=auth_register_failed reason=email_already_registered email_hash={}",
                email_log_hash(email)
            );
            HttpResponse::Conflict().body("Email already registered")
        }
        DieselError::NotFound => {
            log::error!("event=auth_register_failed reason=default_student_role_missing");
            HttpResponse::InternalServerError().body("Failed to register user")
        }
        err => {
            log::error!("event=auth_register_failed reason=database_error error={err}");
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<db::DbPool>, req: web::Json<LoginRequest>) -> impl Responder {
    let email = normalize_email(&req.email);
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let user_auth_result = User::find_with_password_auth(&email, &mut conn).await;

    match user_auth_result {
        Ok((user, info_auth)) => {
            if let Some(hash) = info_auth {
                if verify(&req.password, &hash).unwrap_or(false) {
                    if !user.email_verified {
                        log::info!(
                            "event=auth_login_denied reason=email_unverified user_id={} email_hash={}",
                            user.id(),
                            email_log_hash(&email)
                        );
                        return HttpResponse::Forbidden().body("Email verification required");
                    }

                    match create_jwt(user.id()) {
                        Ok(user_jwt) => {
                            HttpResponse::Ok().json(user_jwt) // Return JWT token in response
                        }
                        Err(err) => {
                            log::error!(
                                "event=auth_jwt_create_failed user_id={} error={}",
                                user.id(),
                                err
                            );
                            HttpResponse::InternalServerError().body("Failed to create JWT")
                        }
                    }
                } else {
                    log::info!(
                        "event=auth_login_failed reason=invalid_credentials email_hash={}",
                        email_log_hash(&email)
                    );
                    HttpResponse::Unauthorized().body("Invalid credentials")
                }
            } else {
                log::warn!(
                    "event=auth_login_failed reason=missing_password_auth email_hash={}",
                    email_log_hash(&email)
                );
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => {
            log::info!(
                "event=auth_login_failed reason=invalid_credentials email_hash={}",
                email_log_hash(&email)
            );
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
    }
}
