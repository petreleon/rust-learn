use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::login::{LoginAuthentication, LoginError, LoginStore};
use crate::models::user::User;

pub struct PostgresLoginStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
}

impl<'conn> PostgresLoginStore<'conn> {
    pub fn new(conn: &'conn mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl LoginStore for PostgresLoginStore<'_> {
    fn find_password_auth(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<Option<LoginAuthentication>, LoginError>> {
        async move {
            match User::find_with_password_auth(&email, self.conn).await {
                Ok((user, password_hash)) => Ok(Some(LoginAuthentication {
                    user_id: user.id(),
                    email_verified: user.email_verified,
                    password_hash,
                })),
                Err(error) => {
                    log::info!("event=auth_login_lookup_failed error={}", error);
                    Ok(None)
                }
            }
        }
        .boxed()
    }
}
