// src/api/authentication.rs
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDate;
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

    let has_lowercase = password.chars().any(char::is_lowercase);
    let has_uppercase = password.chars().any(char::is_uppercase);
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    if !(has_lowercase && has_uppercase && has_digit && has_symbol) {
        return Err("Password must include lowercase, uppercase, numeric, and symbol characters");
    }

    Ok(())
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
                        return HttpResponse::Forbidden().body("Email verification required");
                    }

                    match create_jwt(user.id()) {
                        Ok(user_jwt) => {
                            HttpResponse::Ok().json(user_jwt) // Return JWT token in response
                        }
                        Err(_) => HttpResponse::InternalServerError().body("Failed to create JWT"),
                    }
                } else {
                    HttpResponse::Unauthorized().body("Invalid credentials")
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
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

    // Create new user
    let new_user_data = NewUser {
        name: req.name.to_string(),
        email: req.email.clone(),
        date_of_birth: req.date_of_birth,
        created_at: chrono::Utc::now().naive_utc(),
        kyc_verified: false,
        email_verified: false,
    };

    let inserted_user = User::create(new_user_data, &mut conn)
        .await
        .expect("Error saving new user");

    // Assign default role (STUDENT)
    let role_id = PlatformRole::find_by_name("STUDENT", &mut conn)
        .await
        .expect("Error finding STUDENT role");

    UserRolePlatform::assign(&mut conn, inserted_user.id(), role_id)
        .await
        .expect("Error assigning default role to user");

    // Hash password and create authentication
    let hashed_password = hash(&req.password, DEFAULT_COST).unwrap();
    let new_auth = Authentication {
        user_id: inserted_user.id(),
        type_authentication: "password".to_string(),
        info_auth: hashed_password,
    };

    Authentication::create(new_auth, &mut conn)
        .await
        .expect("Error saving new authentication");

    let verification_token = match generate_verification_token() {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to generate email verification token: {}", err);
            return HttpResponse::InternalServerError()
                .body("Failed to create email verification token");
        }
    };

    let token_hash = verification_token_hash(&verification_token);
    if let Err(err) =
        EmailVerificationToken::create_for_user(&mut conn, inserted_user.id(), token_hash).await
    {
        eprintln!(
            "Failed to save email verification token for user {}: {}",
            inserted_user.id(),
            err
        );
        return HttpResponse::InternalServerError().body("Failed to save email verification token");
    }

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
            eprintln!("Failed to verify email token: {}", err);
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
            eprintln!("Failed to build JWKS response: {}", err);
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
    use super::validate_password_strength;

    #[test]
    fn accepts_strong_password() {
        assert!(validate_password_strength("CorrectHorse1!").is_ok());
    }

    #[test]
    fn rejects_short_password() {
        assert_eq!(
            validate_password_strength("Aa1!").unwrap_err(),
            "Password must be at least 12 characters long"
        );
    }

    #[test]
    fn rejects_password_missing_required_character_classes() {
        assert_eq!(
            validate_password_strength("correcthorse1").unwrap_err(),
            "Password must include lowercase, uppercase, numeric, and symbol characters"
        );
    }
}
