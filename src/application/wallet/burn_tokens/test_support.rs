use bigdecimal::BigDecimal;
use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::burn_tokens::{
    TokenBurnDraft, TokenBurnError, TokenBurnLeaderboard, TokenBurnLeaderboardQuery,
    TokenBurnStore, TokenBurnView,
};
use crate::domain::wallet::burn::{TokenBurnLeaderboardScope, TokenBurnLeaderboardWindow};

#[derive(Default)]
pub(crate) struct FakeTokenBurnStore {
    pub kyc_verified: bool,
    pub organization_exists: bool,
    pub can_burn_organization: bool,
    pub can_view_leaderboard: bool,
    pub deposit_tax: BigDecimal,
    pub created_draft: Option<TokenBurnDraft>,
}

impl TokenBurnStore for FakeTokenBurnStore {
    fn user_kyc_verified(&mut self, _: i32) -> BoxFuture<'_, Result<bool, TokenBurnError>> {
        ready(Ok(self.kyc_verified)).boxed()
    }

    fn organization_exists(&mut self, _: i32) -> BoxFuture<'_, Result<bool, TokenBurnError>> {
        ready(Ok(self.organization_exists)).boxed()
    }

    fn can_burn_organization_tokens(
        &mut self,
        _: i32,
        _: i32,
    ) -> BoxFuture<'_, Result<bool, TokenBurnError>> {
        ready(Ok(self.can_burn_organization)).boxed()
    }

    fn can_view_burn_leaderboard(&mut self, _: i32) -> BoxFuture<'_, Result<bool, TokenBurnError>> {
        ready(Ok(self.can_view_leaderboard)).boxed()
    }

    fn load_platform_deposit_tax(&mut self) -> BoxFuture<'_, Result<BigDecimal, TokenBurnError>> {
        ready(Ok(self.deposit_tax.clone())).boxed()
    }

    fn create_burn_request(
        &mut self,
        draft: TokenBurnDraft,
    ) -> BoxFuture<'_, Result<TokenBurnView, TokenBurnError>> {
        self.created_draft = Some(draft);
        ready(Ok(sample_burn_view())).boxed()
    }

    fn list_user_burns(
        &mut self,
        _: i32,
    ) -> BoxFuture<'_, Result<Vec<TokenBurnView>, TokenBurnError>> {
        ready(Ok(vec![])).boxed()
    }

    fn list_organization_burns(
        &mut self,
        _: i32,
    ) -> BoxFuture<'_, Result<Vec<TokenBurnView>, TokenBurnError>> {
        ready(Ok(vec![])).boxed()
    }

    fn load_leaderboard(
        &mut self,
        _: TokenBurnLeaderboardQuery,
        window: TokenBurnLeaderboardWindow,
        _: TokenBurnLeaderboardScope,
    ) -> BoxFuture<'_, Result<TokenBurnLeaderboard, TokenBurnError>> {
        ready(Ok(TokenBurnLeaderboard {
            window_days: window.days(),
            scope: "all".to_string(),
            rows: vec![],
        }))
        .boxed()
    }
}

fn sample_burn_view() -> TokenBurnView {
    let now = chrono::Utc::now();
    TokenBurnView {
        id: 1,
        actor_user_id: 7,
        burner_type: "user".to_string(),
        user_id: Some(7),
        organization_id: None,
        wallet_id: Some(1),
        source: "centralized_wallet".to_string(),
        fee_path: "none".to_string(),
        status: "leaderboard_indexed".to_string(),
        amount: "10".to_string(),
        fee_amount: "0".to_string(),
        idempotency_key: "key".to_string(),
        deposit_intent_id: None,
        transaction_id: Some(1),
        external_transaction_id: None,
        internal_transaction_id: Some(1),
        permission_evidence: None,
        wallet_provider: "metamask".to_string(),
        metamask_required: false,
        wallet_action: "platform_burn_from_allowance".to_string(),
        leaderboard_visible: true,
        created_at: now,
        updated_at: now,
    }
}
