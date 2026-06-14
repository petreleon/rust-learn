use futures::future::BoxFuture;

use super::{PasswordResetRecipient, RequestPasswordResetError};

pub trait RequestPasswordResetStore {
    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<Option<PasswordResetRecipient>, RequestPasswordResetError>>;

    fn store_reset_token(
        &mut self,
        user_id: i32,
        token: String,
    ) -> BoxFuture<'_, Result<(), RequestPasswordResetError>>;
}
