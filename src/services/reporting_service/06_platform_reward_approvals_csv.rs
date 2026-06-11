pub async fn platform_reward_approvals_csv(conn: &mut AsyncPgConnection) -> QueryResult<String> {
    let candidates = reward_candidates::table
        .filter(
            reward_candidates::teacher_decided_at
                .is_not_null()
                .or(reward_candidates::amount_decided_at.is_not_null()),
        )
        .order(reward_candidates::updated_at.desc())
        .limit(1000)
        .load::<RewardCandidate>(conn)
        .await?;

    let mut csv = String::from(
        "reward_candidate_id,course_id,student_user_id,submitter_user_id,source_scope,source_organization_id,event_type,status,teacher_approver_user_id,teacher_decision_reason,teacher_decided_at,amount_reviewer_user_id,approved_amount,amount_decision_reason,amount_decided_at,created_at,updated_at\n",
    );
    for candidate in candidates {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            candidate.id,
            candidate.course_id,
            candidate.student_user_id,
            candidate.submitter_user_id,
            csv_value(&candidate.source_scope),
            csv_optional(candidate.source_organization_id),
            csv_value(&candidate.event_type),
            csv_value(&candidate.status),
            csv_optional(candidate.teacher_approver_user_id),
            csv_value(candidate.teacher_decision_reason.as_deref().unwrap_or("")),
            csv_optional(candidate.teacher_decided_at),
            csv_optional(candidate.amount_reviewer_user_id),
            csv_value(
                candidate
                    .approved_amount
                    .as_ref()
                    .map(ToString::to_string)
                    .as_deref()
                    .unwrap_or("")
            ),
            csv_value(candidate.amount_decision_reason.as_deref().unwrap_or("")),
            csv_optional(candidate.amount_decided_at),
            candidate.created_at,
            candidate.updated_at
        ));
    }

    Ok(csv)
}

pub async fn platform_token_payouts_csv(conn: &mut AsyncPgConnection) -> QueryResult<String> {
    type ExternalTransactionRow = (
        BigDecimal,
        String,
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<String>,
    );

    let payout_records = reward_payout_records::table
        .order(reward_payout_records::created_at.desc())
        .limit(1000)
        .load::<RewardPayoutRecord>(conn)
        .await?;

    let mut csv = String::from(
        "reward_payout_record_id,reward_candidate_id,course_id,student_user_id,payout_transaction_id,external_transaction_id,amount,blockchain_address,chain_id,contract_address,transaction_hash,log_index,event_type,from_address,to_address,created_at\n",
    );
    for record in payout_records {
        let candidate = reward_candidates::table
            .find(record.reward_candidate_id)
            .first::<RewardCandidate>(conn)
            .await?;
        let external = external_transactions::table
            .find(record.external_transaction_id)
            .select((
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
            .first::<ExternalTransactionRow>(conn)
            .await?;

        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            record.id,
            record.reward_candidate_id,
            candidate.course_id,
            candidate.student_user_id,
            record.transaction_id,
            record.external_transaction_id,
            csv_value(external.0.to_string()),
            csv_value(external.1),
            csv_optional(external.2),
            csv_value(external.3.as_deref().unwrap_or("")),
            csv_value(external.4.as_deref().unwrap_or("")),
            csv_optional(external.5),
            csv_value(external.6.as_deref().unwrap_or("")),
            csv_value(external.7.as_deref().unwrap_or("")),
            csv_value(external.8.as_deref().unwrap_or("")),
            record.created_at
        ));
    }

    Ok(csv)
}

pub async fn platform_wallet_credits_csv(conn: &mut AsyncPgConnection) -> QueryResult<String> {
    let credit_records = reward_wallet_credit_records::table
        .order(reward_wallet_credit_records::created_at.desc())
        .limit(1000)
        .load::<RewardWalletCreditRecord>(conn)
        .await?;

    let mut csv = String::from(
        "reward_wallet_credit_record_id,reward_candidate_id,course_id,student_user_id,wallet_id,transaction_id,internal_transaction_id,amount,notification_id,notified_at,created_at\n",
    );
    for record in credit_records {
        let candidate = reward_candidates::table
            .find(record.reward_candidate_id)
            .first::<RewardCandidate>(conn)
            .await?;
        let amount = internal_transactions::table
            .find(record.internal_transaction_id)
            .select(internal_transactions::amount)
            .first::<BigDecimal>(conn)
            .await?;

        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}\n",
            record.id,
            record.reward_candidate_id,
            candidate.course_id,
            candidate.student_user_id,
            record.wallet_id,
            record.transaction_id,
            record.internal_transaction_id,
            csv_value(amount.to_string()),
            csv_optional(record.notification_id),
            csv_optional(record.notified_at),
            record.created_at
        ));
    }

    Ok(csv)
}
