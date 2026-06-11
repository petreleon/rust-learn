async fn load_internal_transactions(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
) -> QueryResult<Vec<WalletInternalTransactionAudit>> {
    let rows = internal_transactions::table
        .inner_join(
            transactions_internal_transactions::table.on(internal_transactions::id
                .eq(transactions_internal_transactions::internal_transaction_id)),
        )
        .inner_join(
            transactions::table
                .on(transactions_internal_transactions::transaction_id.eq(transactions::id)),
        )
        .filter(internal_transactions::wallet_id.eq(wallet_id))
        .select((
            internal_transactions::id,
            transactions::id,
            transactions::type_,
            internal_transactions::amount,
            transactions::created_at,
        ))
        .order(transactions::created_at.desc())
        .load::<(i64, i64, String, bigdecimal::BigDecimal, DateTime<Utc>)>(conn)
        .await?;

    Ok(rows
        .into_iter()
        .map(
            |(internal_transaction_id, transaction_id, transaction_type, amount, created_at)| {
                WalletInternalTransactionAudit {
                    internal_transaction_id,
                    transaction_id,
                    transaction_type,
                    amount: amount.to_string(),
                    created_at,
                }
            },
        )
        .collect())
}

async fn load_wallet_reward_candidate_ids(
    conn: &mut AsyncPgConnection,
    wallet: &Wallet,
) -> QueryResult<Vec<i64>> {
    let mut candidate_ids = HashSet::new();

    let credit_candidate_ids = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::wallet_id.eq(wallet.id))
        .select(reward_wallet_credit_records::reward_candidate_id)
        .load::<i64>(conn)
        .await?;
    candidate_ids.extend(credit_candidate_ids);

    if let Some(user_id) = wallet.user_id {
        let user_candidate_ids = reward_candidates::table
            .filter(reward_candidates::student_user_id.eq(user_id))
            .select(reward_candidates::id)
            .load::<i64>(conn)
            .await?;
        candidate_ids.extend(user_candidate_ids);
    }

    if let Some(organization_id) = wallet.organization_id {
        let organization_candidate_ids = reward_candidates::table
            .filter(reward_candidates::source_organization_id.eq(organization_id))
            .select(reward_candidates::id)
            .load::<i64>(conn)
            .await?;
        candidate_ids.extend(organization_candidate_ids);
    }

    let mut candidate_ids = candidate_ids.into_iter().collect::<Vec<_>>();
    candidate_ids.sort_unstable();
    Ok(candidate_ids)
}

async fn load_reward_records(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
) -> QueryResult<Vec<WalletRewardRecordAudit>> {
    if candidate_ids.is_empty() {
        return Ok(Vec::new());
    }

    let candidates = reward_candidates::table
        .filter(reward_candidates::id.eq_any(candidate_ids))
        .order(reward_candidates::created_at.desc())
        .load::<RewardCandidate>(conn)
        .await?;

    let credit_records = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq_any(candidate_ids))
        .load::<RewardWalletCreditRecord>(conn)
        .await?
        .into_iter()
        .map(|record| (record.reward_candidate_id, record))
        .collect::<HashMap<_, _>>();

    let payout_records = reward_payout_records::table
        .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids))
        .load::<RewardPayoutRecord>(conn)
        .await?
        .into_iter()
        .map(|record| (record.reward_candidate_id, record))
        .collect::<HashMap<_, _>>();

    Ok(candidates
        .into_iter()
        .map(|candidate| {
            let credit_record = credit_records.get(&candidate.id);
            let payout_record = payout_records.get(&candidate.id);
            WalletRewardRecordAudit {
                reward_candidate_id: candidate.id,
                candidate_status: candidate.status.clone(),
                reconciliation_status: reward_reconciliation_status(
                    &candidate,
                    credit_record,
                    payout_record,
                ),
                approved_amount: candidate.approved_amount.as_ref().map(ToString::to_string),
                wallet_credit_record_id: credit_record.map(|record| record.id),
                wallet_credit_transaction_id: credit_record.map(|record| record.transaction_id),
                internal_transaction_id: credit_record.map(|record| record.internal_transaction_id),
                payout_record_id: payout_record.map(|record| record.id),
                payout_transaction_id: payout_record.map(|record| record.transaction_id),
                external_transaction_id: payout_record.map(|record| record.external_transaction_id),
                notification_id: credit_record.and_then(|record| record.notification_id),
                notified_at: credit_record.and_then(|record| record.notified_at),
                created_at: candidate.created_at,
                updated_at: candidate.updated_at,
            }
        })
        .collect())
}
