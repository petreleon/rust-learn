mod email_verification;
mod forgot_password;
mod jwks;
mod login;
mod password_reset;
mod registration;
mod session;
mod verify_email;

#[cfg(test)]
mod tests;

use actix_web::web;

pub use jwks::jwks;

pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(forgot_password::forgot_password)
        .service(login::login)
        .service(registration::register)
        .service(email_verification::resend_verification)
        .service(password_reset::reset_password)
        .service(verify_email::verify_email)
        .service(session::user_id)
}
