async fn load_external_transactions(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
    wallet_transaction_ids: &[i64],
) -> QueryResult<Vec<WalletExternalTransactionAudit>> {
    let mut audits = Vec::new();
    let mut seen = HashSet::new();

    if !candidate_ids.is_empty() {
        let rows =
            reward_payout_records::table
                .inner_join(external_transactions::table.on(
                    reward_payout_records::external_transaction_id.eq(external_transactions::id),
                ))
                .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids))
                .select((
                    reward_payout_records::reward_candidate_id,
                    reward_payout_records::transaction_id,
                    external_transactions::id,
                    external_transactions::amount,
                    external_transactions::blockchain_address,
                    external_transactions::chain_id,
                    external_transactions::contract_address,
                    external_transactions::transaction_hash,
                    external_transactions::log_index,
                    external_transactions::event_type,
                    external_transactions::from_address,
                    external_transactions::to_address,
                ))
                .order(external_transactions::id.desc())
                .load::<(
                    i64,
                    i64,
                    i64,
                    bigdecimal::BigDecimal,
                    String,
                    Option<i64>,
                    Option<String>,
                    Option<String>,
                    Option<i64>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                )>(conn)
                .await?;

        for (
            reward_candidate_id,
            transaction_id,
            external_transaction_id,
            amount,
            blockchain_address,
            chain_id,
            contract_address,
            transaction_hash,
            log_index,
            event_type,
            from_address,
            to_address,
        ) in rows
        {
            seen.insert((transaction_id, external_transaction_id));
            audits.push(WalletExternalTransactionAudit {
                external_transaction_id,
                transaction_id,
                reward_candidate_id: Some(reward_candidate_id),
                amount: amount.to_string(),
                blockchain_address,
                chain_id,
                contract_address,
                transaction_hash,
                log_index,
                event_type,
                from_address,
                to_address,
            });
        }
    }

    if !wallet_transaction_ids.is_empty() {
        let rows = transactions_external_transactions::table
            .inner_join(
                external_transactions::table
                    .on(transactions_external_transactions::external_transaction_id
                        .eq(external_transactions::id)),
            )
            .filter(
                transactions_external_transactions::transaction_id.eq_any(wallet_transaction_ids),
            )
            .select((
                transactions_external_transactions::transaction_id,
                external_transactions::id,
                external_transactions::amount,
                external_transactions::blockchain_address,
                external_transactions::chain_id,
                external_transactions::contract_address,
                external_transactions::transaction_hash,
                external_transactions::log_index,
                external_transactions::event_type,
                external_transactions::from_address,
                external_transactions::to_address,
            ))
            .order(external_transactions::id.desc())
            .load::<(
                i64,
                i64,
                bigdecimal::BigDecimal,
                String,
                Option<i64>,
                Option<String>,
                Option<String>,
                Option<i64>,
                Option<String>,
                Option<String>,
                Option<String>,
            )>(conn)
            .await?;

        for (
            transaction_id,
            external_transaction_id,
            amount,
            blockchain_address,
            chain_id,
            contract_address,
            transaction_hash,
            log_index,
            event_type,
            from_address,
            to_address,
        ) in rows
        {
            if seen.insert((transaction_id, external_transaction_id)) {
                audits.push(WalletExternalTransactionAudit {
                    external_transaction_id,
                    transaction_id,
                    reward_candidate_id: None,
                    amount: amount.to_string(),
                    blockchain_address,
                    chain_id,
                    contract_address,
                    transaction_hash,
                    log_index,
                    event_type,
                    from_address,
                    to_address,
                });
            }
        }
    }

    audits.sort_by(|left, right| {
        right
            .external_transaction_id
            .cmp(&left.external_transaction_id)
            .then_with(|| right.transaction_id.cmp(&left.transaction_id))
    });

    Ok(audits)
}
