use crate::application::identity::password_policy::validate_password_strength;

use super::{
    RegisterCommand, RegisterError, RegisterOutcome, RegisterStore, RegistrationAccount,
    RegistrationEmailSender, RegistrationPasswordHasher, RegistrationTokenGenerator,
};

pub async fn register(
    store: &mut impl RegisterStore,
    password_hasher: &impl RegistrationPasswordHasher,
    token_generator: &impl RegistrationTokenGenerator,
    email_sender: &impl RegistrationEmailSender,
    command: RegisterCommand,
) -> Result<RegisterOutcome, RegisterError> {
    if let Err(message) = validate_password_strength(&command.password) {
        return Err(RegisterError::InvalidPassword(message.to_string()));
    }

    let password_hash = password_hasher.hash_password(&command.password)?;
    let verification_token = token_generator.generate_token()?;
    let account = RegistrationAccount {
        email: command.email,
        name: command.name,
        date_of_birth: command.date_of_birth,
    };
    let registered_user = store
        .register_account(account, password_hash, verification_token.clone())
        .await?;

    email_sender.send_verification_email(
        &registered_user.email,
        &registered_user.name,
        &verification_token,
    );

    Ok(RegisterOutcome::Registered)
}

#[cfg(test)]
mod tests;
