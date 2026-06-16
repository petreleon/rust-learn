use crate::application::wallet::burn_tokens::TokenBurnView;
use crate::infra::postgres::models::token_burn_request::TokenBurnRequest;

pub(super) fn token_burn_view(record: TokenBurnRequest) -> TokenBurnView {
    TokenBurnView {
        id: record.id,
        actor_user_id: record.actor_user_id,
        burner_type: record.burner_type,
        user_id: record.user_id,
        organization_id: record.organization_id,
        wallet_id: record.wallet_id,
        source: record.source,
        fee_path: record.fee_path,
        status: record.status,
        amount: record.amount.to_string(),
        fee_amount: record.fee_amount.to_string(),
        idempotency_key: record.idempotency_key,
        deposit_intent_id: record.deposit_intent_id,
        transaction_id: record.transaction_id,
        external_transaction_id: record.external_transaction_id,
        internal_transaction_id: record.internal_transaction_id,
        permission_evidence: record.permission_evidence,
        last_error: record.last_error,
        wallet_provider: record.wallet_provider,
        metamask_required: record.metamask_required,
        wallet_action: record.wallet_action,
        leaderboard_visible: record.leaderboard_visible,
        created_at: record.created_at,
        updated_at: record.updated_at,
    }
}
