use crate::db::schema::wallets;
use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = wallets)]
pub struct Wallet {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: BigDecimal,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = wallets)]
pub struct NewWallet {
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: BigDecimal,
}
