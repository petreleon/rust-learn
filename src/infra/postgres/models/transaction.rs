use crate::db::schema::{
    external_transactions, internal_transactions, transactions, transactions_external_transactions,
    transactions_internal_transactions,
};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i64,
    #[diesel(column_name = type_)]
    pub type_: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    #[diesel(column_name = type_)]
    pub type_: &'a str,
}

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = internal_transactions)]
pub struct InternalTransaction {
    pub id: i64,
    pub wallet_id: i32,
    pub amount: BigDecimal,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = internal_transactions)]
pub struct NewInternalTransaction {
    pub wallet_id: i32,
    pub amount: BigDecimal,
}

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = external_transactions)]
pub struct ExternalTransaction {
    pub id: i64,
    pub amount: BigDecimal,
    pub blockchain_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub event_type: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = external_transactions)]
pub struct NewExternalTransaction<'a> {
    pub amount: BigDecimal,
    pub blockchain_address: &'a str,
    pub chain_id: Option<i64>,
    pub contract_address: Option<&'a str>,
    pub transaction_hash: Option<&'a str>,
    pub log_index: Option<i64>,
    pub event_type: Option<&'a str>,
    pub from_address: Option<&'a str>,
    pub to_address: Option<&'a str>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = transactions_internal_transactions)]
pub struct NewTransactionInternalTransactionLink {
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = transactions_external_transactions)]
pub struct NewTransactionExternalTransactionLink {
    pub transaction_id: i64,
    pub external_transaction_id: i64,
}
