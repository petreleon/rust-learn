async fn ensure_internal_transaction_link(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    internal_transaction_id: i64,
) -> QueryResult<bool> {
    let inserted = diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .on_conflict((
            transactions_internal_transactions::transaction_id,
            transactions_internal_transactions::internal_transaction_id,
        ))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(inserted > 0)
}
