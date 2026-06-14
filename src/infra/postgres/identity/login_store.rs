use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::login::{LoginAuthentication, LoginError, LoginStore};
use crate::infra::postgres::identity::accounts::find_password_authentication_by_email;

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
            match find_password_authentication_by_email(self.conn, &email).await {
                Ok(account) => Ok(account.map(|account| LoginAuthentication {
                    user_id: account.user_id,
                    email_verified: account.email_verified,
                    password_hash: account.password_hash,
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
