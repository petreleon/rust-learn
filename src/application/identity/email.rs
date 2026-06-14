use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sha2::{Digest, Sha256};

pub fn email_log_hash(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(normalize_email(email).as_bytes());
    URL_SAFE_NO_PAD
        .encode(hasher.finalize())
        .chars()
        .take(16)
        .collect()
}

pub fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::{email_log_hash, normalize_email};

    #[test]
    fn email_log_hash_normalizes_case_and_redacts_raw_email() {
        let first = email_log_hash(" Learner@Example.COM ");
        let second = email_log_hash("learner@example.com");

        assert_eq!(first, second);
        assert_eq!(first.len(), 16);
        assert!(!first.contains("learner"));
        assert!(!first.contains('@'));
    }

    #[test]
    fn normalize_email_trims_and_lowercases_input() {
        assert_eq!(
            normalize_email(" Learner+Demo@Example.COM "),
            "learner+demo@example.com"
        );
    }

    #[test]
    fn email_log_hash_distinguishes_different_addresses() {
        assert_ne!(
            email_log_hash("learner@example.com"),
            email_log_hash("teacher@example.com")
        );
    }
}
