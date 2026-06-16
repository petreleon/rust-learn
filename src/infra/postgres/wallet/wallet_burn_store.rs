use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};
use std::str::FromStr;

use crate::application::wallet::burn_tokens::{
    TokenBurnDraft, TokenBurnError, TokenBurnLeaderboard, TokenBurnLeaderboardQuery,
    TokenBurnStore, TokenBurnView,
};
use crate::domain::wallet::burn::{TokenBurnLeaderboardScope, TokenBurnLeaderboardWindow};
use crate::infra::postgres::operations::persistent_state::get_persistent_state;
use crate::infra::postgres::schema::{organizations, users};
use crate::infra::postgres::wallet::wallet_access::{
    can_burn_organization_tokens, can_view_burn_leaderboard,
};

const TOKEN_DEPOSIT_TAX_KEY: &str = "wallet.deposit_tax_tokens";

pub struct PostgresTokenBurnStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTokenBurnStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TokenBurnStore for PostgresTokenBurnStore<'_> {
    fn user_kyc_verified(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, TokenBurnError>> {
        async move {
            users::table
                .find(user_id)
                .select(users::kyc_verified)
                .first::<bool>(self.conn)
                .await
                .map_err(|error| TokenBurnError::KycLoad(error.to_string()))
        }
        .boxed()
    }

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, TokenBurnError>> {
        async move {
            organizations::table
                .find(organization_id)
                .select(organizations::id)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map(|row| row.is_some())
                .map_err(|error| TokenBurnError::OrganizationLoad(error.to_string()))
        }
        .boxed()
    }

    fn can_burn_organization_tokens(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, TokenBurnError>> {
        async move {
            can_burn_organization_tokens(self.conn, actor_user_id, organization_id)
                .await
                .map_err(|error| TokenBurnError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }

    fn can_view_burn_leaderboard(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TokenBurnError>> {
        async move {
            can_view_burn_leaderboard(self.conn, actor_user_id)
                .await
                .map_err(|error| TokenBurnError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }

    fn load_platform_deposit_tax(&mut self) -> BoxFuture<'_, Result<BigDecimal, TokenBurnError>> {
        async move {
            let Some(value) = get_persistent_state(self.conn, TOKEN_DEPOSIT_TAX_KEY)
                .await
                .map_err(|error| TokenBurnError::TaxLoad(error.to_string()))?
            else {
                return Ok(BigDecimal::from(0));
            };
            BigDecimal::from_str(value.trim()).map_err(|_| {
                TokenBurnError::TaxLoad("invalid stored deposit token tax amount".to_string())
            })
        }
        .boxed()
    }

    fn create_burn_request(
        &mut self,
        draft: TokenBurnDraft,
    ) -> BoxFuture<'_, Result<TokenBurnView, TokenBurnError>> {
        async move { super::wallet_burn_records::create_burn_request(self.conn, draft).await }
            .boxed()
    }

    fn list_user_burns(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<TokenBurnView>, TokenBurnError>> {
        async move { super::wallet_burn_records::list_user_burns(self.conn, actor_user_id).await }
            .boxed()
    }

    fn list_organization_burns(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Vec<TokenBurnView>, TokenBurnError>> {
        async move {
            super::wallet_burn_records::list_organization_burns(self.conn, organization_id).await
        }
        .boxed()
    }

    fn load_leaderboard(
        &mut self,
        query: TokenBurnLeaderboardQuery,
        window: TokenBurnLeaderboardWindow,
        scope: TokenBurnLeaderboardScope,
    ) -> BoxFuture<'_, Result<TokenBurnLeaderboard, TokenBurnError>> {
        async move {
            super::wallet_burn_leaderboard::load_leaderboard(self.conn, query, window, scope).await
        }
        .boxed()
    }
}
