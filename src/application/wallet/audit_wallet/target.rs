use crate::domain::wallet::owner::WalletOwnerType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletAuditTarget {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: String,
}

impl WalletAuditTarget {
    pub fn owner_type(&self) -> WalletOwnerType {
        WalletOwnerType::from_user_id(self.user_id)
    }
}
