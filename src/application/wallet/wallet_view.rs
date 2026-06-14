#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletView {
    pub id: i32,
    pub owner_type: &'static str,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: String,
}
