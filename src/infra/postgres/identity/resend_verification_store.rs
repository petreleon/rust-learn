use diesel::prelude::*;
use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::resend_verification::{
    ResendVerificationError, ResendVerificationStore, VerificationEmailTarget,
};
use crate::infra::tokens::identity::identity_token_hash;
use crate::models::email_verification_token::EmailVerificationToken;
use crate::models::user::User;

pub struct PostgresResendVerificationStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
}

impl<'conn> PostgresResendVerificationStore<'conn> {
    pub fn new(conn: &'conn mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl ResendVerificationStore for PostgresResendVerificationStore<'_> {
    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<Option<VerificationEmailTarget>, ResendVerificationError>> {
        async move {
            User::find_by_email(&email, self.conn)
                .await
                .optional()
                .map(|user| user.map(map_target))
                .map_err(|error| ResendVerificationError::Lookup(error.to_string()))
        }
        .boxed()
    }

    fn rotate_verification_token(
        &mut self,
        user_id: i32,
        token: String,
    ) -> BoxFuture<'_, Result<(), ResendVerificationError>> {
        async move {
            EmailVerificationToken::create_for_user(self.conn, user_id, identity_token_hash(&token))
                .await
                .map(|_| ())
                .map_err(|error| ResendVerificationError::Store(error.to_string()))
        }
        .boxed()
    }
}

fn map_target(user: User) -> VerificationEmailTarget {
    VerificationEmailTarget {
        user_id: user.id(),
        email: user.email,
        name: user.name,
        email_verified: user.email_verified,
    }
}
