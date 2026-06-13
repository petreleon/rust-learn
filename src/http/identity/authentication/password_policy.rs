const MIN_PASSWORD_LENGTH: usize = 12;
const MAX_BCRYPT_PASSWORD_BYTES: usize = 71;
pub(super) const PASSWORD_TOO_LONG_MESSAGE: &str =
    "Password must be at most 71 UTF-8 bytes for bcrypt hashing";

pub fn validate_password_strength(password: &str) -> Result<(), &'static str> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err("Password must be at least 12 characters long");
    }

    if password.len() > MAX_BCRYPT_PASSWORD_BYTES {
        return Err(PASSWORD_TOO_LONG_MESSAGE);
    }

    let has_lowercase = password.chars().any(char::is_lowercase);
    let has_uppercase = password.chars().any(char::is_uppercase);
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    if !(has_lowercase && has_uppercase && has_digit && has_symbol) {
        return Err("Password must include lowercase, uppercase, numeric, and symbol characters");
    }

    Ok(())
}
