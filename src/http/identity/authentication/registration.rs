use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::application::identity::email::{email_log_hash, normalize_email};
use crate::application::identity::password_policy::validate_password_strength;
use crate::application::identity::register::{
    RegisterCommand, RegisterError, RegisterOutcome, RegisterUseCase,
};

#[derive(Deserialize)]
pub(super) struct RegisterRequest {
    email: String,
    password: String,
    name: String,
    date_of_birth: Option<NaiveDate>,
}

#[post("/register")]
pub(super) async fn register(
    use_case: web::Data<Arc<dyn RegisterUseCase>>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return HttpResponse::BadRequest().body("Email is required");
    }

    if let Err(message) = validate_password_strength(&req.password) {
        return HttpResponse::BadRequest().body(message);
    }

    match use_case
        .register(RegisterCommand {
            email: email.clone(),
            password: req.password.to_string(),
            name: req.name.to_string(),
            date_of_birth: req.date_of_birth,
        })
        .await
    {
        Ok(RegisterOutcome::Registered) => HttpResponse::Ok().body("Registration successful"),
        Err(error) => register_error_response(error, &email),
    }
}

fn register_error_response(error: RegisterError, email: &str) -> HttpResponse {
    match error {
        RegisterError::InvalidPassword(message) => HttpResponse::BadRequest().body(message),
        RegisterError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        RegisterError::PasswordHash(error) => {
            log::error!("event=auth_password_hash_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to register user")
        }
        RegisterError::TokenGeneration(error) => {
            log::error!("event=email_verification_token_generate_failed error={error}");
            HttpResponse::InternalServerError().body("Failed to create email verification token")
        }
        RegisterError::EmailAlreadyRegistered => {
            log::warn!(
                "event=auth_register_failed reason=email_already_registered email_hash={}",
                email_log_hash(email)
            );
            HttpResponse::Conflict().body("Email already registered")
        }
        RegisterError::DefaultStudentRoleMissing => {
            log::error!("event=auth_register_failed reason=default_student_role_missing");
            HttpResponse::InternalServerError().body("Failed to register user")
        }
        RegisterError::Store(error) => {
            log::error!("event=auth_register_failed reason=database_error error={error}");
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}
