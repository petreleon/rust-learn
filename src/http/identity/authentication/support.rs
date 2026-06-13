use actix_web::HttpResponse;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use crate::utils::email::verification_token_hash;

pub(super) fn email_log_hash(email: &str) -> String {
    verification_token_hash(&normalize_email(email))
        .chars()
        .take(16)
        .collect()
}

pub(super) fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

pub(super) fn registration_db_error_response(error: DieselError, email: &str) -> HttpResponse {
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
