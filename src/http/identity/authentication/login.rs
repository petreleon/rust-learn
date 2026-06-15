use std::sync::Arc;

use actix_web::{post, web};
use serde::Deserialize;

use crate::application::identity::email::normalize_email;
use crate::application::identity::login::{LoginCommand, LoginUseCase};
use crate::http::identity::authentication::errors::login_error;
use crate::http::identity::authentication::text_error::AuthTextError;

#[derive(Deserialize)]
pub(super) struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
pub(super) async fn login(
    use_case: web::Data<Arc<dyn LoginUseCase>>,
    req: web::Json<LoginRequest>,
) -> Result<web::Json<String>, AuthTextError> {
    let email = normalize_email(&req.email);

    use_case
        .login(LoginCommand {
            email: email.clone(),
            password: req.password.clone(),
        })
        .await
        .map(|output| web::Json(output.jwt))
        .map_err(|error| login_error(error, &email))
}
