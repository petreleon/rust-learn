pub async fn list_student_reward_history(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: StudentRewardHistoryRequest,
) -> Result<Vec<StudentRewardHistoryEntry>, StudentRewardHistoryError> {
    let status = match request.status.as_deref() {
        Some(status) => Some(normalize_reward_status(status)?),
        None => None,
    };

    let mut query = reward_candidates::table
        .inner_join(courses::table.on(reward_candidates::course_id.eq(courses::id)))
        .filter(reward_candidates::student_user_id.eq(actor_user_id))
        .into_boxed();

    if let Some(course_id) = request.course_id {
        query = query.filter(reward_candidates::course_id.eq(course_id));
    }

    if let Some(status) = status {
        query = query.filter(reward_candidates::status.eq(status));
    }

    let rows = query
        .select((RewardCandidate::as_select(), courses::title))
        .order(reward_candidates::created_at.desc())
        .limit(request.limit())
        .offset(request.offset())
        .load::<(RewardCandidate, String)>(conn)
        .await?;

    let mut history = Vec::new();
    for (candidate, course_title) in rows {
        let can_view_reward_status = user_permission_course_request(
            conn,
            actor_user_id,
            candidate.course_id,
            &Permissions::VIEW_COURSE_REWARD_STATUS.to_string(),
        )
        .await?;

        if !can_view_reward_status {
            continue;
        }

        let wallet_credit = load_wallet_credit(conn, candidate.id).await?;
        let token_transaction = load_token_transaction(conn, candidate.id).await?;
        history.push(StudentRewardHistoryEntry {
            reward_candidate_id: candidate.id,
            course_id: candidate.course_id,
            course_title,
            event_type: candidate.event_type,
            status: candidate.status,
            approved_amount: candidate
                .approved_amount
                .as_ref()
                .map(std::string::ToString::to_string),
            wallet_credit,
            token_transaction,
            created_at: candidate.created_at,
            updated_at: candidate.updated_at,
        });
    }

    Ok(history)
}

async fn load_wallet_credit(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> Result<Option<StudentRewardWalletCredit>, StudentRewardHistoryError> {
    type WalletCreditRow = (
        i64,
        i32,
        i64,
        i64,
        Option<i64>,
        Option<DateTime<Utc>>,
        DateTime<Utc>,
        BigDecimal,
    );

    let row = reward_wallet_credit_records::table
        .inner_join(internal_transactions::table.on(
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transactions::id),
        ))
        .filter(reward_wallet_credit_records::reward_candidate_id.eq(reward_candidate_id))
        .select((
            reward_wallet_credit_records::id,
            reward_wallet_credit_records::wallet_id,
            reward_wallet_credit_records::transaction_id,
            reward_wallet_credit_records::internal_transaction_id,
            reward_wallet_credit_records::notification_id,
            reward_wallet_credit_records::notified_at,
            reward_wallet_credit_records::created_at,
            internal_transactions::amount,
        ))
        .first::<WalletCreditRow>(conn)
        .await
        .optional()?;

    Ok(row.map(
        |(
            reward_wallet_credit_record_id,
            wallet_id,
            transaction_id,
            internal_transaction_id,
            notification_id,
            notified_at,
            credited_at,
            amount,
        )| StudentRewardWalletCredit {
            reward_wallet_credit_record_id,
            wallet_id,
            transaction_id,
            internal_transaction_id,
            amount: amount.to_string(),
            notification_id,
            notified_at,
            credited_at,
        },
    ))
}
