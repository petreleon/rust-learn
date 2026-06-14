use diesel::prelude::*;
use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::request_password_reset::{
    PasswordResetRecipient, RequestPasswordResetError, RequestPasswordResetStore,
};
use crate::models::password_reset_token::PasswordResetToken;
use crate::models::user::User;
use crate::utils::email::verification_token_hash;

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
            User::find_by_email(&email, self.conn)
                .await
                .optional()
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
            PasswordResetToken::create_for_user(self.conn, user_id, verification_token_hash(&token))
                .await
                .map(|_| ())
                .map_err(|error| RequestPasswordResetError::Store(error.to_string()))
        }
        .boxed()
    }
}

fn map_recipient(user: User) -> PasswordResetRecipient {
    PasswordResetRecipient {
        user_id: user.id(),
        email: user.email,
        name: user.name,
    }
}
