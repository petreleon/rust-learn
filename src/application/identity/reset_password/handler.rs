use crate::application::identity::password_policy::validate_password_strength;

use super::{
    ResetPasswordCommand, ResetPasswordError, ResetPasswordHasher, ResetPasswordOutcome,
    ResetPasswordStore,
};

pub async fn reset_password(
    store: &mut impl ResetPasswordStore,
    password_hasher: &impl ResetPasswordHasher,
    command: ResetPasswordCommand,
) -> Result<ResetPasswordOutcome, ResetPasswordError> {
    let token = command.token.trim();
    if token.is_empty() {
        return Err(ResetPasswordError::MissingToken);
    }
    if let Err(message) = validate_password_strength(&command.password) {
        return Err(ResetPasswordError::InvalidPassword(message.to_string()));
    }

    let password_hash = password_hasher.hash_password(&command.password)?;
    store.reset_password(token.to_string(), password_hash).await
}

#[cfg(test)]
mod tests;
