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

impl ExternalTransaction {
    pub fn create(
        new_external_transaction: NewExternalTransaction<'_>,
        conn: &mut PgConnection,
    ) -> QueryResult<i64> {
        diesel::insert_into(external_transactions::table)
            .values(&new_external_transaction)
            .returning(external_transactions::id)
            .get_result(conn)
    }

    pub fn find_by_chain_tx_log(
        chain_id: i64,
        transaction_hash: &str,
        log_index: i64,
        conn: &mut PgConnection,
    ) -> QueryResult<Option<ExternalTransaction>> {
        external_transactions::table
            .filter(external_transactions::chain_id.eq(chain_id))
            .filter(external_transactions::transaction_hash.eq(transaction_hash))
            .filter(external_transactions::log_index.eq(log_index))
            .first(conn)
            .optional()
    }
}

pub struct TransactionLink;

impl TransactionLink {
    pub fn create(
        transaction_id: i64,
        internal_transaction_id: i64,
        conn: &mut PgConnection,
    ) -> QueryResult<usize> {
        diesel::insert_into(transactions_internal_transactions::table)
            .values(NewTransactionInternalTransactionLink {
                transaction_id,
                internal_transaction_id,
            })
            .execute(conn)
    }

    pub fn create_external(
        transaction_id: i64,
        external_transaction_id: i64,
        conn: &mut PgConnection,
    ) -> QueryResult<usize> {
        diesel::insert_into(transactions_external_transactions::table)
            .values(NewTransactionExternalTransactionLink {
                transaction_id,
                external_transaction_id,
            })
            .execute(conn)
    }

    pub fn find_transaction_for_external(
        external_transaction_id: i64,
        conn: &mut PgConnection,
    ) -> QueryResult<Option<i64>> {
        transactions_external_transactions::table
            .filter(
                transactions_external_transactions::external_transaction_id
                    .eq(external_transaction_id),
            )
            .select(transactions_external_transactions::transaction_id)
            .first(conn)
            .optional()
    }
}
