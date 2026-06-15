use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::domain::access_control::permission::Permission;

#[derive(Default)]
pub(crate) struct FakeWalletAuthorizationStore {
    pub platform_permissions: Vec<Permission>,
    pub organization_permissions: Vec<(i32, Permission)>,
    pub platform_checks: Vec<Permission>,
    pub organization_checks: Vec<(i32, Permission)>,
}

impl WalletAuthorizationStore for FakeWalletAuthorizationStore {
    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>> {
        let permission = permission_from_action(&action);
        let allowed = match scope {
            AccessScope::Platform(_) => {
                self.platform_checks.push(permission);
                self.platform_permissions.contains(&permission)
            }
            AccessScope::Organization(scope) => {
                self.organization_checks
                    .push((scope.organization_id(), permission));
                self.organization_permissions
                    .contains(&(scope.organization_id(), permission))
            }
            AccessScope::Course(_) => false,
        };

        ready(Ok(allowed)).boxed()
    }
}

fn permission_from_action(action: &AccessAction) -> Permission {
    match action.permission_name() {
        "CREATE_WALLET" => Permission::CreateWallet,
        "MANAGE_ORG_REWARD_BUDGET" => Permission::ManageOrgRewardBudget,
        "MANAGE_ORG_WALLETS" => Permission::ManageOrgWallets,
        "MANAGE_WALLETS" => Permission::ManageWallets,
        "RECONCILE_WALLETS" => Permission::ReconcileWallets,
        "SET_DEPOSIT_TAX" => Permission::SetDepositTax,
        "SET_RETIRE_TAX" => Permission::SetRetireTax,
        "VIEW_ORG_REWARD_REPORTS" => Permission::ViewOrgRewardReports,
        "VIEW_TRANSACTIONS" => Permission::ViewTransactions,
        "VIEW_WALLET" => Permission::ViewWallet,
        permission => panic!("unsupported fake wallet permission: {permission}"),
    }
}
