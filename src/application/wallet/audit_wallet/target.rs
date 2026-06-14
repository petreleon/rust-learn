#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletAuditTarget {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: String,
}

impl WalletAuditTarget {
    pub fn owner_type(&self) -> &'static str {
        if self.user_id.is_some() {
            "user"
        } else {
            "organization"
        }
    }
}
