#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletAuthorizationAction {
    ViewUserWallet,
    ViewOrganizationWallet { organization_id: i32 },
    LinkUserWallet,
    LinkOrganizationWallet { organization_id: i32 },
    BurnOrganizationTokens { organization_id: i32 },
    ViewBurnLeaderboard,
    ReconcileTokenBurns,
    SetDepositTax,
    SetRetireTax,
}
