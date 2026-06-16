use bigdecimal::BigDecimal;

use crate::application::wallet::burn_tokens::validation::{
    ensure_fee_path_matches_source, status_for_source, validate_command,
};
use crate::application::wallet::burn_tokens::{
    TokenBurnCommand, TokenBurnDraft, TokenBurnError, TokenBurnLeaderboard,
    TokenBurnLeaderboardQuery, TokenBurnStore, TokenBurnSubject, TokenBurnView,
};
use crate::domain::wallet::burn::{
    TokenBurnFeePath, TokenBurnLeaderboardScope, TokenBurnLeaderboardWindow, TokenBurnSource,
    TokenBurnerType,
};

pub async fn request_token_burn(
    store: &mut impl TokenBurnStore,
    actor_user_id: i32,
    subject: TokenBurnSubject,
    command: TokenBurnCommand,
) -> Result<TokenBurnView, TokenBurnError> {
    if !store.user_kyc_verified(actor_user_id).await? {
        return Err(TokenBurnError::KycRequired);
    }
    validate_command(&command)?;
    let source = TokenBurnSource::parse(&command.source)
        .map_err(|error| TokenBurnError::InvalidInput(error.to_string()))?;
    let fee_path = TokenBurnFeePath::parse(&command.fee_path)
        .map_err(|error| TokenBurnError::InvalidInput(error.to_string()))?;
    ensure_fee_path_matches_source(source, fee_path)?;
    ensure_subject_allowed(store, actor_user_id, subject).await?;

    let fee_amount = match fee_path {
        TokenBurnFeePath::PlatformDepositFee => store.load_platform_deposit_tax().await?,
        _ => BigDecimal::from(0),
    };
    let (burner_type, user_id, organization_id, permission_evidence) =
        burn_attribution(actor_user_id, subject);
    let status = status_for_source(&command, source);

    store
        .create_burn_request(TokenBurnDraft {
            actor_user_id,
            burner_type,
            user_id,
            organization_id,
            amount: command.amount,
            fee_amount,
            source,
            fee_path,
            status,
            idempotency_key: command.idempotency_key.trim().to_string(),
            ethereum_address: command.ethereum_address.as_deref().map(normalize),
            platform_address: command.platform_address.as_deref().map(normalize),
            chain_id: command.chain_id,
            contract_address: command.contract_address.as_deref().map(normalize),
            transaction_hash: command.transaction_hash.as_deref().map(normalize),
            log_index: command.log_index,
            deposit_intent_id: command.deposit_intent_id,
            permission_evidence,
            wallet_provider: "metamask".to_string(),
            metamask_required: source != TokenBurnSource::CentralizedWallet,
            wallet_action: wallet_action(source).to_string(),
            leaderboard_visible: command.leaderboard_visible.unwrap_or(true),
        })
        .await
}

pub async fn list_token_burns(
    store: &mut impl TokenBurnStore,
    actor_user_id: i32,
    subject: TokenBurnSubject,
) -> Result<Vec<TokenBurnView>, TokenBurnError> {
    match subject {
        TokenBurnSubject::OwnUser => store.list_user_burns(actor_user_id).await,
        TokenBurnSubject::Organization(organization_id) => {
            ensure_subject_allowed(store, actor_user_id, subject).await?;
            store.list_organization_burns(organization_id).await
        }
    }
}

pub async fn load_token_burn_leaderboard(
    store: &mut impl TokenBurnStore,
    actor_user_id: i32,
    query: TokenBurnLeaderboardQuery,
) -> Result<TokenBurnLeaderboard, TokenBurnError> {
    if !store.can_view_burn_leaderboard(actor_user_id).await? {
        return Err(TokenBurnError::PermissionDenied);
    }
    let window = TokenBurnLeaderboardWindow::parse(&query.window)
        .map_err(|error| TokenBurnError::InvalidInput(error.to_string()))?;
    let scope = TokenBurnLeaderboardScope::parse(query.scope.as_deref())
        .map_err(|error| TokenBurnError::InvalidInput(error.to_string()))?;
    store.load_leaderboard(query, window, scope).await
}

async fn ensure_subject_allowed(
    store: &mut impl TokenBurnStore,
    actor_user_id: i32,
    subject: TokenBurnSubject,
) -> Result<(), TokenBurnError> {
    match subject {
        TokenBurnSubject::OwnUser => Ok(()),
        TokenBurnSubject::Organization(organization_id) => {
            if !store.organization_exists(organization_id).await? {
                return Err(TokenBurnError::OrganizationNotFound);
            }
            if store
                .can_burn_organization_tokens(actor_user_id, organization_id)
                .await?
            {
                Ok(())
            } else {
                Err(TokenBurnError::PermissionDenied)
            }
        }
    }
}

fn burn_attribution(
    actor_user_id: i32,
    subject: TokenBurnSubject,
) -> (TokenBurnerType, Option<i32>, Option<i32>, Option<String>) {
    match subject {
        TokenBurnSubject::OwnUser => (TokenBurnerType::User, Some(actor_user_id), None, None),
        TokenBurnSubject::Organization(organization_id) => (
            TokenBurnerType::Organization,
            None,
            Some(organization_id),
            Some("BURN_ORGANIZATION_TOKENS".to_string()),
        ),
    }
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn wallet_action(source: TokenBurnSource) -> &'static str {
    match source {
        TokenBurnSource::CentralizedWallet => "platform_burn_from_allowance",
        TokenBurnSource::DecentralizedDirect => "metamask_burn",
        TokenBurnSource::DecentralizedPlatformMediated => "metamask_deposit_then_platform_burn",
    }
}
