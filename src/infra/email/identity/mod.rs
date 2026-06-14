use std::env;

const DEFAULT_APP_PUBLIC_URL: &str = "http://localhost:8080/api/auth";
const DEFAULT_WEB_PUBLIC_URL: &str = "http://localhost:3000";

fn public_url_from_env_value(value: Option<&str>, default_url: &str) -> String {
    value
        .map(str::trim)
        .map(|value| value.trim_end_matches('/'))
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| default_url.to_string())
}

fn app_public_url_from_env_value(value: Option<&str>) -> String {
    public_url_from_env_value(value, DEFAULT_APP_PUBLIC_URL)
}

fn app_public_url_from_env() -> String {
    app_public_url_from_env_value(env::var("APP_PUBLIC_URL").ok().as_deref())
}

fn web_public_url_from_env() -> String {
    public_url_from_env_value(
        env::var("WEB_PUBLIC_URL").ok().as_deref(),
        DEFAULT_WEB_PUBLIC_URL,
    )
}

pub fn verification_url(token: &str) -> String {
    format!("{}/verify-email?token={}", app_public_url_from_env(), token)
}

pub fn password_reset_url(token: &str) -> String {
    format!(
        "{}/reset-password?token={}",
        web_public_url_from_env(),
        token
    )
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

pub fn build_mock_password_reset_email(to: &str, name: &str, reset_url: &str) -> String {
    format!(
        "\
================ MOCK EMAIL ================
To: {to}
Subject: Reset your RustLearn password

Hi {name},

Use this password reset link to choose a new RustLearn password:
{reset_url}

This is a local-development email preview printed by the API. No email was sent.
============================================
"
    )
}

pub fn print_mock_password_reset_email(to: &str, name: &str, token: &str) {
    let reset_url = password_reset_url(token);
    println!("{}", build_mock_password_reset_email(to, name, &reset_url));
}

#[cfg(test)]
mod tests {
    use super::{
        app_public_url_from_env_value, build_mock_password_reset_email,
        build_mock_verification_email, password_reset_url, verification_url,
        DEFAULT_APP_PUBLIC_URL,
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
    fn builds_terminal_verification_mock_email() {
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
    fn builds_terminal_password_reset_mock_email() {
        let email = build_mock_password_reset_email(
            "learner@example.com",
            "Demo Learner",
            "http://localhost:3000/reset-password?token=mock-preview-token",
        );

        assert!(email.contains("To: learner@example.com"));
        assert!(email.contains("Subject: Reset your RustLearn password"));
        assert!(email.contains("Hi Demo Learner,"));
        assert!(email.contains("mock-preview-token"));
        assert!(email.contains("No email was sent."));
    }

    #[test]
    fn builds_auth_and_password_reset_urls_from_base() {
        assert!(verification_url("mock-preview-token")
            .ends_with("/verify-email?token=mock-preview-token"));
        assert!(password_reset_url("mock-preview-token")
            .ends_with("/reset-password?token=mock-preview-token"));
    }
}
