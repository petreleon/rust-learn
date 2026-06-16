use chrono::{DateTime, Utc};

use crate::application::wallet::burn_tokens::TokenBurnDraft;
use crate::domain::wallet::burn::TokenBurnStatus;
use crate::infra::postgres::models::token_burn_request::NewTokenBurnRequest;

pub(super) fn new_burn_request(
    draft: TokenBurnDraft,
    wallet_id: i32,
    transaction_id: i64,
    external_transaction_id: Option<i64>,
    internal_transaction_id: Option<i64>,
    now: DateTime<Utc>,
) -> NewTokenBurnRequest {
    let indexed = draft.status == TokenBurnStatus::LeaderboardIndexed;
    NewTokenBurnRequest {
        actor_user_id: draft.actor_user_id,
        burner_type: draft.burner_type.as_str().to_string(),
        user_id: draft.user_id,
        organization_id: draft.organization_id,
        wallet_id: Some(wallet_id),
        source: draft.source.as_str().to_string(),
        fee_path: draft.fee_path.as_str().to_string(),
        status: draft.status.as_str().to_string(),
        amount: draft.amount,
        fee_amount: draft.fee_amount,
        idempotency_key: draft.idempotency_key,
        deposit_intent_id: draft.deposit_intent_id,
        transaction_id: Some(transaction_id),
        external_transaction_id,
        internal_transaction_id,
        permission_evidence: draft.permission_evidence,
        wallet_provider: draft.wallet_provider,
        metamask_required: draft.metamask_required,
        wallet_action: draft.wallet_action,
        leaderboard_visible: draft.leaderboard_visible,
        confirmed_at: indexed.then_some(now),
        ledger_recorded_at: internal_transaction_id.map(|_| now),
        leaderboard_indexed_at: indexed.then_some(now),
    }
}
