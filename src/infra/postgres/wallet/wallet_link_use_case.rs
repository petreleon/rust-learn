use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::link_wallet::{
    self, LinkedWalletView, WalletLinkError, WalletLinkSubject, WalletLinkUseCase,
};
use crate::infra::postgres::wallet::wallet_link_store::PostgresWalletLinkStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresWalletLinkUseCase {
    pool: DbPool,
}

impl PostgresWalletLinkUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl WalletLinkUseCase for PostgresWalletLinkUseCase {
    fn link_wallet(
        &self,
        actor_user_id: i32,
        subject: WalletLinkSubject,
    ) -> BoxFuture<'_, Result<LinkedWalletView, WalletLinkError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletLinkStore::new(&mut conn);
            link_wallet::link_wallet(&mut store, actor_user_id, subject).await
        }
        .boxed()
    }
}

impl PostgresWalletLinkUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, WalletLinkError> {
        self.pool
            .get()
            .await
            .map_err(|error| WalletLinkError::Connection(error.to_string()))
    }
}
