use diesel::prelude::*;
use crate::db::schema::{transactions, internal_transactions, transactions_internal_transactions};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

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

#[derive(Insertable, Debug)]
#[diesel(table_name = transactions_internal_transactions)]
pub struct NewTransactionInternalTransactionLink {
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
}

impl Transaction {
    pub fn create(type_: &str, conn: &mut PgConnection) -> QueryResult<i64> {
        diesel::insert_into(transactions::table)
            .values(NewTransaction { type_ })
            .returning(transactions::id)
            .get_result(conn)
    }
}

impl InternalTransaction {
    pub fn create(wallet_id: i32, amount: BigDecimal, conn: &mut PgConnection) -> QueryResult<i64> {
        diesel::insert_into(internal_transactions::table)
            .values(NewInternalTransaction { wallet_id, amount })
            .returning(internal_transactions::id)
            .get_result(conn)
    }
}

pub struct TransactionLink;

impl TransactionLink {
    pub fn create(transaction_id: i64, internal_transaction_id: i64, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::insert_into(transactions_internal_transactions::table)
            .values(NewTransactionInternalTransactionLink { transaction_id, internal_transaction_id })
            .execute(conn)
    }
}
