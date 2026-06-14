use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sha2::{Digest, Sha256};

pub fn generate_identity_token() -> Result<String, getrandom::Error> {
    let mut token_bytes = [0_u8; 32];
    getrandom::getrandom(&mut token_bytes)?;
    Ok(URL_SAFE_NO_PAD.encode(token_bytes))
}

pub fn identity_token_hash(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::{generate_identity_token, identity_token_hash};

    #[test]
    fn generates_url_safe_identity_token() {
        let token = generate_identity_token().expect("token should generate");

        assert_eq!(token.len(), 43);
        assert!(token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn hashes_identity_token_consistently() {
        let first = identity_token_hash("mock-preview-token");
        let second = identity_token_hash("mock-preview-token");

        assert_eq!(first, second);
        assert_ne!(first, "mock-preview-token");
    }
}
