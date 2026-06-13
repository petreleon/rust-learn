async fn ensure_external_transaction_link(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    external_transaction_id: i64,
) -> QueryResult<bool> {
    let inserted = diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .on_conflict((
            transactions_external_transactions::transaction_id,
            transactions_external_transactions::external_transaction_id,
        ))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(inserted > 0)
}
