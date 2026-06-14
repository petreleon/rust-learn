use futures::future::BoxFuture;

use super::{ResendVerificationError, VerificationEmailTarget};

pub trait ResendVerificationStore {
    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<Option<VerificationEmailTarget>, ResendVerificationError>>;

    fn rotate_verification_token(
        &mut self,
        user_id: i32,
        token: String,
    ) -> BoxFuture<'_, Result<(), ResendVerificationError>>;
}
