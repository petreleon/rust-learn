use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::wallet::burn_tokens::{
    TokenBurnDraft, TokenBurnError, TokenBurnLeaderboard, TokenBurnLeaderboardQuery, TokenBurnView,
};
use crate::domain::wallet::burn::{TokenBurnLeaderboardScope, TokenBurnLeaderboardWindow};

pub trait TokenBurnStore {
    fn user_kyc_verified(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, TokenBurnError>>;

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, TokenBurnError>>;

    fn can_burn_organization_tokens(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, TokenBurnError>>;

    fn can_view_burn_leaderboard(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TokenBurnError>>;

    fn load_platform_deposit_tax(&mut self) -> BoxFuture<'_, Result<BigDecimal, TokenBurnError>>;

    fn create_burn_request(
        &mut self,
        draft: TokenBurnDraft,
    ) -> BoxFuture<'_, Result<TokenBurnView, TokenBurnError>>;

    fn list_user_burns(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<TokenBurnView>, TokenBurnError>>;

    fn list_organization_burns(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Vec<TokenBurnView>, TokenBurnError>>;

    fn load_leaderboard(
        &mut self,
        query: TokenBurnLeaderboardQuery,
        window: TokenBurnLeaderboardWindow,
        scope: TokenBurnLeaderboardScope,
    ) -> BoxFuture<'_, Result<TokenBurnLeaderboard, TokenBurnError>>;
}
