#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    CreateWallet,
    ExecuteRewardPayout,
    ManageOrgRewardBudget,
    ManageOrgWallets,
    ManageWallets,
    ReconcileWallets,
    SetDepositTax,
    SetRetireTax,
    ViewOrgRewardReports,
    ViewTransactions,
    ViewWallet,
}

impl Permission {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreateWallet => "CREATE_WALLET",
            Self::ExecuteRewardPayout => "EXECUTE_REWARD_PAYOUT",
            Self::ManageOrgRewardBudget => "MANAGE_ORG_REWARD_BUDGET",
            Self::ManageOrgWallets => "MANAGE_ORG_WALLETS",
            Self::ManageWallets => "MANAGE_WALLETS",
            Self::ReconcileWallets => "RECONCILE_WALLETS",
            Self::SetDepositTax => "SET_DEPOSIT_TAX",
            Self::SetRetireTax => "SET_RETIRE_TAX",
            Self::ViewOrgRewardReports => "VIEW_ORG_REWARD_REPORTS",
            Self::ViewTransactions => "VIEW_TRANSACTIONS",
            Self::ViewWallet => "VIEW_WALLET",
        }
    }
}
