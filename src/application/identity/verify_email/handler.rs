use super::{VerifyEmailCommand, VerifyEmailError, VerifyEmailOutcome, VerifyEmailStore};

pub async fn verify_email(
    store: &mut impl VerifyEmailStore,
    command: VerifyEmailCommand,
) -> Result<VerifyEmailOutcome, VerifyEmailError> {
    store.verify_email_token(command.token).await
}

#[cfg(test)]
mod tests;
