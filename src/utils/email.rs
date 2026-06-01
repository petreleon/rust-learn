use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sha2::{Digest, Sha256};
use std::env;

const DEFAULT_APP_PUBLIC_URL: &str = "http://localhost:8080/api/auth";

fn app_public_url_from_env_value(value: Option<&str>) -> String {
    value
        .map(str::trim)
        .map(|value| value.trim_end_matches('/'))
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| DEFAULT_APP_PUBLIC_URL.to_string())
}

pub fn app_public_url_from_env() -> String {
    app_public_url_from_env_value(env::var("APP_PUBLIC_URL").ok().as_deref())
}

pub fn generate_verification_token() -> Result<String, getrandom::Error> {
    let mut token_bytes = [0_u8; 32];
    getrandom::getrandom(&mut token_bytes)?;
    Ok(URL_SAFE_NO_PAD.encode(token_bytes))
}

pub fn verification_token_hash(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

pub fn verification_url(token: &str) -> String {
    format!("{}/verify-email?token={}", app_public_url_from_env(), token)
}

pub fn build_mock_verification_email(to: &str, name: &str, verification_url: &str) -> String {
    format!(
        "\
================ MOCK EMAIL ================
To: {to}
Subject: Confirm your RustLearn account

Hi {name},

Welcome to RustLearn. Use this account verification link:
{verification_url}

This is a local-development email preview printed by the API. No email was sent.
============================================
"
    )
}

pub fn print_mock_verification_email(to: &str, name: &str, token: &str) {
    let verification_url = verification_url(token);
    println!(
        "{}",
        build_mock_verification_email(to, name, &verification_url)
    );
}

#[cfg(test)]
mod tests {
    use super::{
        app_public_url_from_env_value, build_mock_verification_email, generate_verification_token,
        verification_token_hash, verification_url, DEFAULT_APP_PUBLIC_URL,
    };

    #[test]
    fn defaults_app_public_url_when_missing_or_blank() {
        assert_eq!(app_public_url_from_env_value(None), DEFAULT_APP_PUBLIC_URL);
        assert_eq!(
            app_public_url_from_env_value(Some("   ")),
            DEFAULT_APP_PUBLIC_URL
        );
        assert_eq!(
            app_public_url_from_env_value(Some("///")),
            DEFAULT_APP_PUBLIC_URL
        );
    }

    #[test]
    fn trims_app_public_url_trailing_slashes() {
        assert_eq!(
            app_public_url_from_env_value(Some(" https://learn.example.com/// ")),
            "https://learn.example.com"
        );
    }

    #[test]
    fn builds_terminal_mock_email() {
        let email = build_mock_verification_email(
            "learner@example.com",
            "Demo Learner",
            "http://localhost:8080/api/auth/verify-email?token=mock-preview-token",
        );

        assert!(email.contains("To: learner@example.com"));
        assert!(email.contains("Subject: Confirm your RustLearn account"));
        assert!(email.contains("Hi Demo Learner,"));
        assert!(email.contains("mock-preview-token"));
        assert!(email.contains("No email was sent."));
    }

    #[test]
    fn generates_url_safe_verification_token() {
        let token = generate_verification_token().expect("token should generate");

        assert_eq!(token.len(), 43);
        assert!(token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn hashes_verification_token_consistently() {
        let first = verification_token_hash("mock-preview-token");
        let second = verification_token_hash("mock-preview-token");

        assert_eq!(first, second);
        assert_ne!(first, "mock-preview-token");
    }

    #[test]
    fn builds_verification_url_from_base() {
        assert!(verification_url("mock-preview-token")
            .ends_with("/verify-email?token=mock-preview-token"));
    }
}
