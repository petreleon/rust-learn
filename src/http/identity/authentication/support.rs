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
