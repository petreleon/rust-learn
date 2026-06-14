use futures::future::BoxFuture;

use super::{ResetPasswordError, ResetPasswordOutcome};

pub trait ResetPasswordStore {
    fn reset_password(
        &mut self,
        token: String,
        password_hash: String,
    ) -> BoxFuture<'_, Result<ResetPasswordOutcome, ResetPasswordError>>;
}
