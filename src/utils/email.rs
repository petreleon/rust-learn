use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sha2::{Digest, Sha256};

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

#[cfg(test)]
mod tests {
    use super::{generate_verification_token, verification_token_hash};

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
}
