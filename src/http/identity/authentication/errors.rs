use crate::application::identity::email::{email_log_hash, normalize_email};
use crate::application::identity::jwks::JwksError;
use crate::application::identity::login::LoginError;
use crate::application::identity::register::RegisterError;
use crate::application::identity::request_password_reset::RequestPasswordResetError;
use crate::application::identity::resend_verification::ResendVerificationError;
use crate::application::identity::reset_password::ResetPasswordError;
use crate::application::identity::verify_email::VerifyEmailError;
use crate::http::identity::authentication::text_error::AuthTextError;

pub(super) fn missing_email_error() -> AuthTextError {
    AuthTextError::bad_request("Email is required")
}

pub(super) fn login_error(error: LoginError, email: &str) -> AuthTextError {
    let email = normalize_email(email);
    match error {
        LoginError::EmailUnverified { user_id } => {
            log::info!(
                "event=auth_login_denied reason=email_unverified user_id={} email_hash={}",
                user_id,
                email_log_hash(&email)
            );
            AuthTextError::forbidden("Email verification required")
        }
        LoginError::InvalidCredentials => {
            log::info!(
                "event=auth_login_failed reason=invalid_credentials email_hash={}",
                email_log_hash(&email)
            );
            AuthTextError::unauthorized("Invalid credentials")
        }
        LoginError::MissingPasswordAuthentication => {
            log::warn!(
                "event=auth_login_failed reason=missing_password_auth email_hash={}",
                email_log_hash(&email)
            );
            AuthTextError::unauthorized("Invalid credentials")
        }
        LoginError::Connection(_) => AuthTextError::internal("Failed to get DB connection"),
        LoginError::Token { user_id, message } => {
            log::error!(
                "event=auth_jwt_create_failed user_id={} error={}",
                user_id,
                message
            );
            AuthTextError::internal("Failed to create JWT")
        }
    }
}

pub(super) fn register_error(error: RegisterError, email: &str) -> AuthTextError {
    match error {
        RegisterError::InvalidPassword(message) => AuthTextError::bad_request(message),
        RegisterError::Connection(_) => AuthTextError::internal("Failed to get DB connection"),
        RegisterError::PasswordHash(error) => {
            log::error!("event=auth_password_hash_failed error={error}");
            AuthTextError::internal("Failed to register user")
        }
        RegisterError::TokenGeneration(error) => {
            log::error!("event=email_verification_token_generate_failed error={error}");
            AuthTextError::internal("Failed to create email verification token")
        }
        RegisterError::EmailAlreadyRegistered => {
            log::warn!(
                "event=auth_register_failed reason=email_already_registered email_hash={}",
                email_log_hash(email)
            );
            AuthTextError::conflict("Email already registered")
        }
        RegisterError::DefaultStudentRoleMissing => {
            log::error!("event=auth_register_failed reason=default_student_role_missing");
            AuthTextError::internal("Failed to register user")
        }
        RegisterError::Store(error) => {
            log::error!("event=auth_register_failed reason=database_error error={error}");
            AuthTextError::internal("Failed to register user")
        }
    }
}

pub(super) fn request_password_reset_error(error: RequestPasswordResetError) -> AuthTextError {
    match error {
        RequestPasswordResetError::Connection(_) => {
            AuthTextError::internal("Failed to get DB connection")
        }
        RequestPasswordResetError::Lookup(error) => {
            log::error!("event=password_reset_lookup_failed error={error}");
            AuthTextError::internal("Failed to request password reset")
        }
        RequestPasswordResetError::TokenGeneration(error) => {
            log::error!("event=password_reset_token_generate_failed error={error}");
            AuthTextError::internal("Failed to create password reset token")
        }
        RequestPasswordResetError::Store(error) => {
            log::error!("event=password_reset_token_store_failed error={error}");
            AuthTextError::internal("Failed to request password reset")
        }
    }
}

pub(super) fn resend_verification_error(error: ResendVerificationError) -> AuthTextError {
    match error {
        ResendVerificationError::Connection(_) => {
            AuthTextError::internal("Failed to get DB connection")
        }
        ResendVerificationError::Lookup(error) => {
            log::error!("event=email_verification_resend_lookup_failed error={error}");
            AuthTextError::internal("Failed to resend verification email")
        }
        ResendVerificationError::TokenGeneration(error) => {
            log::error!("event=email_verification_resend_token_failed error={error}");
            AuthTextError::internal("Failed to create email verification token")
        }
        ResendVerificationError::Store(error) => {
            log::error!("event=email_verification_resend_store_failed error={error}");
            AuthTextError::internal("Failed to resend verification email")
        }
    }
}

pub(super) fn reset_password_error(error: ResetPasswordError) -> AuthTextError {
    match error {
        ResetPasswordError::MissingToken => {
            AuthTextError::bad_request("Password reset token is required")
        }
        ResetPasswordError::InvalidPassword(message) => AuthTextError::bad_request(message),
        ResetPasswordError::Connection(_) => AuthTextError::internal("Failed to get DB connection"),
        ResetPasswordError::PasswordHash(error) => {
            log::error!("event=password_reset_hash_failed error={error}");
            AuthTextError::internal("Failed to reset password")
        }
        ResetPasswordError::Store(error) => {
            log::error!("event=password_reset_failed error={error}");
            AuthTextError::internal("Failed to reset password")
        }
    }
}

pub(super) fn verify_email_error(error: VerifyEmailError) -> AuthTextError {
    log::error!(
        "event=email_verification_failed error={}",
        match error {
            VerifyEmailError::Connection(message) | VerifyEmailError::Database(message) => message,
        }
    );
    AuthTextError::internal("Failed to verify email token")
}

pub(super) fn jwks_error(error: JwksError) -> AuthTextError {
    log::error!("event=jwks_build_failed error={}", error.message());
    AuthTextError::internal("Failed to build JWKS response")
}
