use super::{
    PasswordResetEmailSender, PasswordResetTokenGenerator, RequestPasswordResetCommand,
    RequestPasswordResetError, RequestPasswordResetOutcome, RequestPasswordResetStore,
};

pub async fn request_password_reset(
    store: &mut impl RequestPasswordResetStore,
    token_generator: &impl PasswordResetTokenGenerator,
    email_sender: &impl PasswordResetEmailSender,
    command: RequestPasswordResetCommand,
) -> Result<RequestPasswordResetOutcome, RequestPasswordResetError> {
    let Some(recipient) = store.find_user_by_email(command.email).await? else {
        return Ok(RequestPasswordResetOutcome::UnknownEmail);
    };

    let token = token_generator.generate_token()?;
    store
        .store_reset_token(recipient.user_id, token.clone())
        .await?;
    email_sender.send_password_reset_email(&recipient.email, &recipient.name, &token);

    Ok(RequestPasswordResetOutcome::Sent)
}

#[cfg(test)]
mod tests;
