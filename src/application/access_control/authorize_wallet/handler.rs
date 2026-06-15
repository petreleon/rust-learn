use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationAction, WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::domain::access_control::permission::Permission;

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
        WalletAuthorizationAction::SetDepositTax => {
            can_platform(store, actor, Permission::SetDepositTax).await
        }
        WalletAuthorizationAction::SetRetireTax => {
            can_platform(store, actor, Permission::SetRetireTax).await
        }
    }
}

async fn can_platform(
    store: &mut impl WalletAuthorizationStore,
    actor: AccessActor,
    permission: Permission,
) -> Result<bool, WalletAuthorizationError> {
    store
        .can(
            actor,
            AccessAction::permission(permission.as_str()),
            AccessScope::platform(),
        )
        .await
}

async fn can_organization(
    store: &mut impl WalletAuthorizationStore,
    actor: AccessActor,
    organization_id: i32,
    permission: Permission,
) -> Result<bool, WalletAuthorizationError> {
    store
        .can(
            actor,
            AccessAction::permission(permission.as_str()),
            AccessScope::organization(organization_id),
        )
        .await
}

async fn has_any_platform_permission(
    store: &mut impl WalletAuthorizationStore,
    actor: AccessActor,
    permissions: &[Permission],
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
    permissions: &[Permission],
) -> Result<bool, WalletAuthorizationError> {
    for &permission in permissions {
        if can_organization(store, actor, organization_id, permission).await? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn user_wallet_view_permissions() -> &'static [Permission] {
    &[
        Permission::ViewWallet,
        Permission::ViewTransactions,
        Permission::ReconcileWallets,
        Permission::ManageWallets,
    ]
}

fn organization_wallet_view_permissions() -> &'static [Permission] {
    &[
        Permission::ManageOrgWallets,
        Permission::ViewOrgRewardReports,
        Permission::ManageOrgRewardBudget,
    ]
}

fn user_wallet_link_permissions() -> &'static [Permission] {
    &[Permission::CreateWallet, Permission::ManageWallets]
}

fn organization_wallet_link_permissions() -> &'static [Permission] {
    &[Permission::ManageOrgWallets]
}
