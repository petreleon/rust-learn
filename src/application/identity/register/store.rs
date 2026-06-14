use futures::future::BoxFuture;

use super::{RegisterError, RegisteredUser, RegistrationAccount};

pub trait RegisterStore {
    fn register_account(
        &mut self,
        account: RegistrationAccount,
        password_hash: String,
        verification_token: String,
    ) -> BoxFuture<'_, Result<RegisteredUser, RegisterError>>;
}
