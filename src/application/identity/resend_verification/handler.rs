use super::{
    ResendVerificationCommand, ResendVerificationError, ResendVerificationOutcome,
    ResendVerificationStore, VerificationEmailSender, VerificationTokenGenerator,
};

pub async fn resend_verification(
    store: &mut impl ResendVerificationStore,
    token_generator: &impl VerificationTokenGenerator,
    email_sender: &impl VerificationEmailSender,
    command: ResendVerificationCommand,
) -> Result<ResendVerificationOutcome, ResendVerificationError> {
    let Some(target) = store.find_user_by_email(command.email).await? else {
        return Ok(ResendVerificationOutcome::UnknownEmail);
    };
    if target.email_verified {
        return Ok(ResendVerificationOutcome::AlreadyVerified);
    }

    let token = token_generator.generate_token()?;
    store
        .rotate_verification_token(target.user_id, token.clone())
        .await?;
    email_sender.send_verification_email(&target.email, &target.name, &token);

    Ok(ResendVerificationOutcome::Sent)
}

#[cfg(test)]
mod tests;
