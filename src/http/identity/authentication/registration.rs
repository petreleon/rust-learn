use std::sync::Arc;

use actix_web::{post, web};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::application::identity::email::normalize_email;
use crate::application::identity::password_policy::validate_password_strength;
use crate::application::identity::register::{RegisterCommand, RegisterOutcome, RegisterUseCase};
use crate::http::identity::authentication::errors::{missing_email_error, register_error};
use crate::http::identity::authentication::text_error::AuthTextError;

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
) -> Result<&'static str, AuthTextError> {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return Err(missing_email_error());
    }

    if let Err(message) = validate_password_strength(&req.password) {
        return Err(AuthTextError::bad_request(message));
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
        Ok(RegisterOutcome::Registered) => Ok("Registration successful"),
        Err(error) => Err(register_error(error, &email)),
    }
}
