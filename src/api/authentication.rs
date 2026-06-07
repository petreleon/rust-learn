// src/api/authentication.rs
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bcrypt::{non_truncating_hash, verify, DEFAULT_COST};
use chrono::NaiveDate;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::AsyncConnection;
use serde::Deserialize;

use crate::db;
use crate::models::authentication::Authentication;
use crate::models::email_verification_token::EmailVerificationToken;
use crate::models::role::PlatformRole;
use crate::models::user::{NewUser, User};
use crate::models::user_role_platform::UserRolePlatform;
use crate::utils::email::{
    generate_verification_token, print_mock_verification_email, verification_token_hash,
};
use crate::utils::jwt_utils::{create_jwt, public_jwks_from_env};

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

fn validate_password_strength(password: &str) -> Result<(), &'static str> {
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
    verification_token_hash(&email.trim().to_ascii_lowercase())
        .chars()
        .take(16)
        .collect()
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
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let user_auth_result = User::find_with_password_auth(&req.email, &mut conn).await;

    match user_auth_result {
        Ok((user, info_auth)) => {
            if let Some(hash) = info_auth {
                if verify(&req.password, &hash).unwrap_or(false) {
                    if !user.email_verified {
                        log::warn!(
                            "event=auth_login_denied reason=email_unverified user_id={} email_hash={}",
                            user.id(),
                            email_log_hash(&user.email)
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
                    log::warn!(
                        "event=auth_login_failed reason=invalid_credentials email_hash={}",
                        email_log_hash(&req.email)
                    );
                    HttpResponse::Unauthorized().body("Invalid credentials")
                }
            } else {
                log::warn!(
                    "event=auth_login_failed reason=missing_password_auth email_hash={}",
                    email_log_hash(&req.email)
                );
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => {
            log::warn!(
                "event=auth_login_failed reason=invalid_credentials email_hash={}",
                email_log_hash(&req.email)
            );
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
    }
}

#[post("/register")]
pub async fn register(
    pool: web::Data<db::DbPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    if let Err(message) = validate_password_strength(&req.password) {
        return HttpResponse::BadRequest().body(message);
    }

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let hashed_password = match non_truncating_hash(&req.password, DEFAULT_COST) {
        Ok(password_hash) => password_hash,
        Err(err) => {
            log::error!("event=auth_password_hash_failed error={}", err);
            return HttpResponse::InternalServerError().body("Failed to register user");
        }
    };

    let verification_token = match generate_verification_token() {
        Ok(token) => token,
        Err(err) => {
            log::error!(
                "event=email_verification_token_generate_failed error={}",
                err
            );
            return HttpResponse::InternalServerError()
                .body("Failed to create email verification token");
        }
    };
    let token_hash = verification_token_hash(&verification_token);

    let new_user_data = NewUser {
        name: req.name.to_string(),
        email: req.email.clone(),
        date_of_birth: req.date_of_birth,
        created_at: chrono::Utc::now().naive_utc(),
        kyc_verified: false,
        email_verified: false,
    };

    let inserted_user = match conn
        .transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let inserted_user = User::create(new_user_data, conn).await?;

                let role_id = PlatformRole::find_by_name("STUDENT", conn).await?;
                UserRolePlatform::assign(conn, inserted_user.id(), role_id).await?;

                let new_auth = Authentication {
                    user_id: inserted_user.id(),
                    type_authentication: "password".to_string(),
                    info_auth: hashed_password,
                };
                Authentication::create(new_auth, conn).await?;

                EmailVerificationToken::create_for_user(conn, inserted_user.id(), token_hash)
                    .await?;

                Ok(inserted_user)
            })
        })
        .await
    {
        Ok(user) => user,
        Err(err) => {
            return registration_db_error_response(err, &req.email);
        }
    };

    print_mock_verification_email(
        &inserted_user.email,
        &inserted_user.name,
        &verification_token,
    );

    HttpResponse::Ok().body("Registration successful")
}

#[get("/verify-email")]
pub async fn verify_email(
    pool: web::Data<db::DbPool>,
    query: web::Query<VerifyEmailQuery>,
) -> impl Responder {
    let token = query.token.trim();
    if token.is_empty() {
        return HttpResponse::BadRequest().body("Verification token is required");
    }

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let token_hash = verification_token_hash(token);
    match EmailVerificationToken::verify(&mut conn, &token_hash).await {
        Ok(true) => HttpResponse::Ok().body("Email verified successfully"),
        Ok(false) => HttpResponse::BadRequest().body("Invalid or expired verification token"),
        Err(err) => {
            log::error!("event=email_verification_failed error={}", err);
            HttpResponse::InternalServerError().body("Failed to verify email token")
        }
    }
}

// hello
#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user_id")]
pub async fn user_id(req: HttpRequest) -> impl Responder {
    // Try to decode the Authorization header to extract the user ID instead of reading request extensions
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(token_data) = crate::utils::jwt_utils::decode_jwt(token) {
                    let user_jwt = token_data.claims;
                    return HttpResponse::Ok()
                        .body(format!("Hello! Your ID is {}", user_jwt.user_id));
                }
            }
        }
    }

    HttpResponse::Ok().body("You didn't provide any ID")
}

pub async fn jwks() -> impl Responder {
    match public_jwks_from_env() {
        Ok(jwks) => HttpResponse::Ok().json(jwks),
        Err(err) => {
            log::error!("event=jwks_build_failed error={}", err);
            HttpResponse::InternalServerError().body("Failed to build JWKS response")
        }
    }
}

// Define the scope for authentication-related routes
pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(login)
        .service(register)
        .service(verify_email)
        .service(hello)
        .service(user_id)
}

#[cfg(test)]
mod tests {
    use super::{email_log_hash, validate_password_strength, PASSWORD_TOO_LONG_MESSAGE};

    #[test]
    fn accepts_strong_password() {
        assert!(validate_password_strength("CorrectHorse1!").is_ok());
    }

    #[test]
    fn accepts_minimum_length_password_with_required_character_classes() {
        assert!(validate_password_strength("Aa1!aaaaaaaa").is_ok());
    }

    #[test]
    fn rejects_short_password() {
        assert_eq!(
            validate_password_strength("Aa1!").unwrap_err(),
            "Password must be at least 12 characters long"
        );
    }

    #[test]
    fn rejects_passwords_missing_required_character_classes() {
        let cases = [
            "CORRECTHORSE1!",
            "correcthorse1!",
            "CorrectHorse!!",
            "CorrectHorse12",
        ];

        for password in cases {
            assert_eq!(
                validate_password_strength(password).unwrap_err(),
                "Password must include lowercase, uppercase, numeric, and symbol characters"
            );
        }
    }

    #[test]
    fn rejects_passwords_over_bcrypt_byte_limit() {
        let long_password = format!("Aa1!{}", "a".repeat(68));

        assert_eq!(
            validate_password_strength(&long_password).unwrap_err(),
            PASSWORD_TOO_LONG_MESSAGE
        );
    }

    #[test]
    fn accepts_password_at_bcrypt_byte_limit() {
        let password = format!("Aa1!{}", "a".repeat(67));

        assert_eq!(password.len(), 71);
        assert!(validate_password_strength(&password).is_ok());
    }

    #[test]
    fn email_log_hash_normalizes_case_and_redacts_raw_email() {
        let first = email_log_hash(" Learner@Example.COM ");
        let second = email_log_hash("learner@example.com");

        assert_eq!(first, second);
        assert_eq!(first.len(), 16);
        assert!(!first.contains("learner"));
        assert!(!first.contains('@'));
    }

    #[test]
    fn email_log_hash_distinguishes_different_addresses() {
        assert_ne!(
            email_log_hash("learner@example.com"),
            email_log_hash("teacher@example.com")
        );
    }
}
