use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::burn_tokens::{
    self, OrganizationTokenBurnPermissions, TokenBurnCommand, TokenBurnError, TokenBurnLeaderboard,
    TokenBurnLeaderboardQuery, TokenBurnSubject, TokenBurnUseCase, TokenBurnView,
};
use crate::infra::postgres::wallet::wallet_burn_store::PostgresTokenBurnStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresTokenBurnUseCase {
    pool: DbPool,
}

impl PostgresTokenBurnUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TokenBurnUseCase for PostgresTokenBurnUseCase {
    fn request_token_burn(
        &self,
        actor_user_id: i32,
        subject: TokenBurnSubject,
        command: TokenBurnCommand,
    ) -> BoxFuture<'_, Result<TokenBurnView, TokenBurnError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTokenBurnStore::new(&mut conn);
            burn_tokens::request_token_burn(&mut store, actor_user_id, subject, command).await
        }
        .boxed()
    }

    fn list_token_burns(
        &self,
        actor_user_id: i32,
        subject: TokenBurnSubject,
    ) -> BoxFuture<'_, Result<Vec<TokenBurnView>, TokenBurnError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTokenBurnStore::new(&mut conn);
            burn_tokens::list_token_burns(&mut store, actor_user_id, subject).await
        }
        .boxed()
    }

    fn load_token_burn_leaderboard(
        &self,
        actor_user_id: i32,
        query: TokenBurnLeaderboardQuery,
    ) -> BoxFuture<'_, Result<TokenBurnLeaderboard, TokenBurnError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTokenBurnStore::new(&mut conn);
            burn_tokens::load_token_burn_leaderboard(&mut store, actor_user_id, query).await
        }
        .boxed()
    }

    fn load_organization_token_burn_permissions(
        &self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationTokenBurnPermissions, TokenBurnError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTokenBurnStore::new(&mut conn);
            burn_tokens::load_organization_token_burn_permissions(
                &mut store,
                actor_user_id,
                organization_id,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresTokenBurnUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TokenBurnError> {
        self.pool
            .get()
            .await
            .map_err(|error| TokenBurnError::Connection(error.to_string()))
    }
}
