mod email_verification;
mod jwks;
mod login;
mod password_policy;
mod password_reset;
mod registration;
mod session;
mod support;
mod verify_email;

#[cfg(test)]
mod tests;

use actix_web::web;

pub use jwks::jwks;
pub use password_policy::validate_password_strength;

pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(password_reset::forgot_password)
        .service(login::login)
        .service(registration::register)
        .service(email_verification::resend_verification)
        .service(password_reset::reset_password)
        .service(verify_email::verify_email)
        .service(session::user_id)
}
