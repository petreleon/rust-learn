use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationAction, WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn authorize_wallet_action(
    store: &mut impl WalletAuthorizationStore,
    actor_user_id: i32,
    action: WalletAuthorizationAction,
) -> Result<bool, WalletAuthorizationError> {
    let actor = AccessActor::user(actor_user_id);
    match action {
        WalletAuthorizationAction::ViewUserWallet => {
            has_any_platform_permission(store, actor, user_wallet_view_permissions()).await
        }
        WalletAuthorizationAction::ViewOrganizationWallet { organization_id } => {
            if has_any_platform_permission(store, actor, user_wallet_view_permissions()).await? {
                return Ok(true);
            }
            has_any_organization_permission(
                store,
                actor,
                organization_id,
                organization_wallet_view_permissions(),
            )
            .await
        }
        WalletAuthorizationAction::LinkUserWallet => {
            has_any_platform_permission(store, actor, user_wallet_link_permissions()).await
        }
        WalletAuthorizationAction::LinkOrganizationWallet { organization_id } => {
            if has_any_platform_permission(store, actor, user_wallet_link_permissions()).await? {
                return Ok(true);
            }
            has_any_organization_permission(
                store,
                actor,
                organization_id,
                organization_wallet_link_permissions(),
            )
            .await
        }
        WalletAuthorizationAction::BurnOrganizationTokens { organization_id } => {
            if can_platform(store, actor, Permissions::MANAGE_WALLETS).await? {
                return Ok(true);
            }
            can_organization(
                store,
                actor,
                organization_id,
                Permissions::BURN_ORGANIZATION_TOKENS,
            )
            .await
        }
        WalletAuthorizationAction::ViewBurnLeaderboard => {
            has_any_platform_permission(store, actor, burn_leaderboard_permissions()).await
        }
        WalletAuthorizationAction::ReconcileTokenBurns => {
            can_platform(store, actor, Permissions::RECONCILE_TOKEN_BURNS).await
        }
        WalletAuthorizationAction::SetDepositTax => {
            can_platform(store, actor, Permissions::SET_DEPOSIT_TAX).await
        }
        WalletAuthorizationAction::SetRetireTax => {
            can_platform(store, actor, Permissions::SET_RETIRE_TAX).await
        }
    }
}

async fn can_platform(
    store: &mut impl WalletAuthorizationStore,
    actor: AccessActor,
    permission: Permissions,
) -> Result<bool, WalletAuthorizationError> {
    store
        .can(
            actor,
            AccessAction::permission(permission),
            AccessScope::platform(),
        )
        .await
}

async fn can_organization(
    store: &mut impl WalletAuthorizationStore,
    actor: AccessActor,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, WalletAuthorizationError> {
    store
        .can(
            actor,
            AccessAction::permission(permission),
            AccessScope::organization(organization_id),
        )
        .await
}

async fn has_any_platform_permission(
    store: &mut impl WalletAuthorizationStore,
    actor: AccessActor,
    permissions: &[Permissions],
) -> Result<bool, WalletAuthorizationError> {
    for &permission in permissions {
        if can_platform(store, actor, permission).await? {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn has_any_organization_permission(
    store: &mut impl WalletAuthorizationStore,
    actor: AccessActor,
    organization_id: i32,
    permissions: &[Permissions],
) -> Result<bool, WalletAuthorizationError> {
    for &permission in permissions {
        if can_organization(store, actor, organization_id, permission).await? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn user_wallet_view_permissions() -> &'static [Permissions] {
    &[
        Permissions::VIEW_WALLET,
        Permissions::VIEW_TRANSACTIONS,
        Permissions::RECONCILE_WALLETS,
        Permissions::MANAGE_WALLETS,
    ]
}

fn organization_wallet_view_permissions() -> &'static [Permissions] {
    &[
        Permissions::MANAGE_ORG_WALLETS,
        Permissions::VIEW_ORG_REWARD_REPORTS,
        Permissions::MANAGE_ORG_REWARD_BUDGET,
    ]
}

fn user_wallet_link_permissions() -> &'static [Permissions] {
    &[Permissions::CREATE_WALLET, Permissions::MANAGE_WALLETS]
}

fn organization_wallet_link_permissions() -> &'static [Permissions] {
    &[Permissions::MANAGE_ORG_WALLETS]
}

fn burn_leaderboard_permissions() -> &'static [Permissions] {
    &[
        Permissions::VIEW_BURN_LEADERBOARD,
        Permissions::VIEW_FINANCIAL_REPORTS,
        Permissions::RECONCILE_WALLETS,
    ]
}
