use crate::domain::wallet::owner::WalletOwnerType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletView {
    pub id: i32,
    pub owner_type: WalletOwnerType,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: String,
}

pub(crate) struct WalletViewFact {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: String,
}

pub(crate) fn wallet_view_output(fact: WalletViewFact) -> WalletView {
    WalletView {
        id: fact.id,
        owner_type: owner_type(fact.user_id),
        user_id: fact.user_id,
        organization_id: fact.organization_id,
        value: fact.value,
    }
}

fn owner_type(user_id: Option<i32>) -> WalletOwnerType {
    WalletOwnerType::from_user_id(user_id)
}
