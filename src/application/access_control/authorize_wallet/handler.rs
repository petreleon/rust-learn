use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationAction, WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;

pub async fn authorize_wallet_action(
    store: &mut impl WalletAuthorizationStore,
    actor_user_id: i32,
    action: WalletAuthorizationAction,
) -> Result<bool, WalletAuthorizationError> {
    match action {
        WalletAuthorizationAction::ViewUserWallet => {
            has_any_platform_permission(store, actor_user_id, user_wallet_view_permissions()).await
        }
        WalletAuthorizationAction::ViewOrganizationWallet { organization_id } => {
            if has_any_platform_permission(store, actor_user_id, user_wallet_view_permissions())
                .await?
            {
                return Ok(true);
            }
            has_any_organization_permission(
                store,
                actor_user_id,
                organization_id,
                organization_wallet_view_permissions(),
            )
            .await
        }
        WalletAuthorizationAction::LinkUserWallet => {
            has_any_platform_permission(store, actor_user_id, user_wallet_link_permissions()).await
        }
        WalletAuthorizationAction::LinkOrganizationWallet { organization_id } => {
            if has_any_platform_permission(store, actor_user_id, user_wallet_link_permissions())
                .await?
            {
                return Ok(true);
            }
            has_any_organization_permission(
                store,
                actor_user_id,
                organization_id,
                organization_wallet_link_permissions(),
            )
            .await
        }
        WalletAuthorizationAction::SetDepositTax => {
            store
                .has_platform_permission(actor_user_id, Permission::SetDepositTax)
                .await
        }
        WalletAuthorizationAction::SetRetireTax => {
            store
                .has_platform_permission(actor_user_id, Permission::SetRetireTax)
                .await
        }
    }
}

async fn has_any_platform_permission(
    store: &mut impl WalletAuthorizationStore,
    actor_user_id: i32,
    permissions: &[Permission],
) -> Result<bool, WalletAuthorizationError> {
    for &permission in permissions {
        if store
            .has_platform_permission(actor_user_id, permission)
            .await?
        {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn has_any_organization_permission(
    store: &mut impl WalletAuthorizationStore,
    actor_user_id: i32,
    organization_id: i32,
    permissions: &[Permission],
) -> Result<bool, WalletAuthorizationError> {
    for &permission in permissions {
        if store
            .has_organization_permission(actor_user_id, organization_id, permission)
            .await?
        {
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
