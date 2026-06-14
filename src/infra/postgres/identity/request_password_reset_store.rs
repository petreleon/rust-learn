use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::request_password_reset::{
    PasswordResetRecipient, RequestPasswordResetError, RequestPasswordResetStore,
};
use crate::infra::postgres::identity::accounts::{
    find_identity_user_by_email, IdentityUserAccount,
};
use crate::infra::postgres::identity::password_reset_tokens::create_password_reset_token;
use crate::infra::tokens::identity::identity_token_hash;

pub struct PostgresRequestPasswordResetStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
}

impl<'conn> PostgresRequestPasswordResetStore<'conn> {
    pub fn new(conn: &'conn mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RequestPasswordResetStore for PostgresRequestPasswordResetStore<'_> {
    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<Option<PasswordResetRecipient>, RequestPasswordResetError>> {
        async move {
            find_identity_user_by_email(self.conn, &email)
                .await
                .map(|user| user.map(map_recipient))
                .map_err(|error| RequestPasswordResetError::Lookup(error.to_string()))
        }
        .boxed()
    }

    fn store_reset_token(
        &mut self,
        user_id: i32,
        token: String,
    ) -> BoxFuture<'_, Result<(), RequestPasswordResetError>> {
        async move {
            create_password_reset_token(self.conn, user_id, identity_token_hash(&token))
                .await
                .map(|_| ())
                .map_err(|error| RequestPasswordResetError::Store(error.to_string()))
        }
        .boxed()
    }
}

fn map_recipient(user: IdentityUserAccount) -> PasswordResetRecipient {
    PasswordResetRecipient {
        user_id: user.user_id,
        email: user.email,
        name: user.name,
    }
}
