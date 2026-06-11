pub async fn platform_wallet_reconciliation(
    conn: &mut AsyncPgConnection,
) -> QueryResult<PlatformWalletReconciliation> {
    let wallet_rows = wallets::table
        .order(wallets::id.desc())
        .load::<Wallet>(conn)
        .await?;

    let mut rows = Vec::new();
    let mut total_internal_transactions = 0i64;
    let mut total_external_transactions = 0i64;
    let mut total_reward_records = 0i64;
    let mut total_needs_reconciliation = 0i64;

    for wallet in wallet_rows {
        let internal_count: i64 = internal_transactions::table
            .filter(internal_transactions::wallet_id.eq(wallet.id))
            .count()
            .get_result(conn)
            .await?;

        let credit_candidate_ids: Vec<i64> = reward_wallet_credit_records::table
            .filter(reward_wallet_credit_records::wallet_id.eq(wallet.id))
            .select(reward_wallet_credit_records::reward_candidate_id)
            .load(conn)
            .await?;

        let mut candidate_ids: std::collections::HashSet<i64> =
            credit_candidate_ids.into_iter().collect();
        if let Some(user_id) = wallet.user_id {
            let ids = reward_candidates::table
                .filter(reward_candidates::student_user_id.eq(user_id))
                .select(reward_candidates::id)
                .load::<i64>(conn)
                .await?;
            candidate_ids.extend(ids);
        }
        if let Some(organization_id) = wallet.organization_id {
            let ids = reward_candidates::table
                .filter(reward_candidates::source_organization_id.eq(organization_id))
                .select(reward_candidates::id)
                .load::<i64>(conn)
                .await?;
            candidate_ids.extend(ids);
        }
        let candidate_ids_vec: Vec<i64> = candidate_ids.into_iter().collect();

        let reward_count = candidate_ids_vec.len() as i64;

        let external_count: i64 = if candidate_ids_vec.is_empty() {
            0
        } else {
            reward_payout_records::table
                .filter(reward_payout_records::reward_candidate_id.eq_any(&candidate_ids_vec))
                .count()
                .get_result(conn)
                .await?
        };

        let needs_reconciliation: i64 = if candidate_ids_vec.is_empty() {
            0
        } else {
            reward_candidates::table
                .filter(reward_candidates::id.eq_any(&candidate_ids_vec))
                .filter(reward_candidates::status.eq(REWARD_STATUS_NEEDS_RECONCILIATION))
                .count()
                .get_result(conn)
                .await?
        };

        let missing_credits: i64 = if candidate_ids_vec.is_empty() {
            0
        } else {
            reward_candidates::table
                .filter(reward_candidates::id.eq_any(&candidate_ids_vec))
                .filter(
                    reward_candidates::status
                        .eq(REWARD_STATUS_WALLET_CREDITED)
                        .or(reward_candidates::status.eq(REWARD_STATUS_NOTIFIED))
                        .or(reward_candidates::status.eq(REWARD_STATUS_COMPLETED)),
                )
                .left_join(reward_wallet_credit_records::table.on(
                    reward_candidates::id.eq(reward_wallet_credit_records::reward_candidate_id),
                ))
                .filter(reward_wallet_credit_records::id.is_null())
                .count()
                .get_result(conn)
                .await?
        };

        let missing_notifications: i64 = if candidate_ids_vec.is_empty() {
            0
        } else {
            reward_candidates::table
                .filter(reward_candidates::id.eq_any(&candidate_ids_vec))
                .filter(
                    reward_candidates::status
                        .eq(REWARD_STATUS_NOTIFIED)
                        .or(reward_candidates::status.eq(REWARD_STATUS_COMPLETED)),
                )
                .left_join(reward_wallet_credit_records::table.on(
                    reward_candidates::id.eq(reward_wallet_credit_records::reward_candidate_id),
                ))
                .filter(
                    reward_wallet_credit_records::notification_id.is_null().or(
                        reward_wallet_credit_records::notification_id
                            .is_null()
                            .and(reward_wallet_credit_records::id.is_not_null()),
                    ),
                )
                .count()
                .get_result(conn)
                .await?
        };

        let missing_payouts: i64 = if candidate_ids_vec.is_empty() {
            0
        } else {
            reward_candidates::table
                .filter(reward_candidates::id.eq_any(&candidate_ids_vec))
                .filter(reward_candidates::status.eq(REWARD_STATUS_TOKEN_CONFIRMED))
                .left_join(
                    reward_payout_records::table
                        .on(reward_candidates::id.eq(reward_payout_records::reward_candidate_id)),
                )
                .filter(reward_payout_records::id.is_null())
                .count()
                .get_result(conn)
                .await?
        };

        total_internal_transactions += internal_count;
        total_external_transactions += external_count;
        total_reward_records += reward_count;
        total_needs_reconciliation += needs_reconciliation;

        rows.push(PlatformWalletReconciliationRow {
            wallet_id: wallet.id,
            owner_type: if wallet.user_id.is_some() {
                "user".to_string()
            } else {
                "organization".to_string()
            },
            user_id: wallet.user_id,
            organization_id: wallet.organization_id,
            balance: wallet.value.to_string(),
            internal_transaction_count: internal_count,
            external_transaction_count: external_count,
            reward_record_count: reward_count,
            needs_reconciliation_count: needs_reconciliation,
            missing_credit_count: missing_credits,
            missing_notification_count: missing_notifications,
            missing_payout_count: missing_payouts,
        });
    }

    Ok(PlatformWalletReconciliation {
        total_wallets: rows.len() as i64,
        total_internal_transactions,
        total_external_transactions,
        total_reward_records,
        total_needs_reconciliation,
        wallets: rows,
    })
}
