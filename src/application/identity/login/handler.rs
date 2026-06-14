use super::{
    LoginCommand, LoginError, LoginOutput, LoginStore, LoginTokenIssuer, PasswordVerifier,
};

pub async fn login(
    store: &mut impl LoginStore,
    password_verifier: &impl PasswordVerifier,
    token_issuer: &impl LoginTokenIssuer,
    command: LoginCommand,
) -> Result<LoginOutput, LoginError> {
    let Some(authentication) = store.find_password_auth(command.email.clone()).await? else {
        return Err(LoginError::InvalidCredentials);
    };
    let Some(password_hash) = authentication.password_hash.as_deref() else {
        return Err(LoginError::MissingPasswordAuthentication);
    };
    if !password_verifier.verify_password(&command.password, password_hash) {
        return Err(LoginError::InvalidCredentials);
    }
    if !authentication.email_verified {
        return Err(LoginError::EmailUnverified {
            user_id: authentication.user_id,
        });
    }

    Ok(LoginOutput {
        jwt: token_issuer.issue_token(authentication.user_id)?,
    })
}

#[cfg(test)]
mod tests;
