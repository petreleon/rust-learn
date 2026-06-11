async fn teacher_application_dashboard_summary(
    conn: &mut AsyncPgConnection,
) -> QueryResult<TeacherApplicationDashboardSummary> {
    let rows = teacher_applications::table
        .group_by(teacher_applications::status)
        .select((teacher_applications::status, diesel::dsl::count_star()))
        .load::<(String, i64)>(conn)
        .await?;

    let mut summary = TeacherApplicationDashboardSummary::default();
    for (status, count) in rows {
        summary.total += count;
        match status.as_str() {
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted = count,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes = count,
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved = count,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected = count,
            _ => {}
        }
    }

    Ok(summary)
}

async fn reward_candidate_dashboard_summary(
    conn: &mut AsyncPgConnection,
) -> QueryResult<RewardCandidateDashboardSummary> {
    let rows = reward_candidates::table
        .group_by(reward_candidates::status)
        .select((reward_candidates::status, diesel::dsl::count_star()))
        .load::<(String, i64)>(conn)
        .await?;

    let mut summary = RewardCandidateDashboardSummary::default();
    for (status, count) in rows {
        summary.total += count;
        match status.as_str() {
            REWARD_STATUS_PENDING_TEACHER_APPROVAL => summary.pending_teacher_approval = count,
            REWARD_STATUS_TEACHER_APPROVED => summary.teacher_approved = count,
            REWARD_STATUS_TEACHER_REJECTED => summary.teacher_rejected = count,
            REWARD_STATUS_AMOUNT_APPROVED => summary.amount_approved = count,
            REWARD_STATUS_AMOUNT_REJECTED => summary.amount_rejected = count,
            REWARD_STATUS_TOKEN_PENDING => summary.token_pending = count,
            REWARD_STATUS_TOKEN_CONFIRMED => summary.token_confirmed = count,
            REWARD_STATUS_WALLET_CREDITED => summary.wallet_credited = count,
            REWARD_STATUS_NOTIFIED => summary.notified = count,
            REWARD_STATUS_COMPLETED => summary.completed = count,
            REWARD_STATUS_NEEDS_RECONCILIATION => summary.needs_reconciliation = count,
            REWARD_STATUS_FAILED => summary.failed = count,
            _ => {}
        }
    }

    Ok(summary)
}

async fn reward_reconciliation_mismatches(
    conn: &mut AsyncPgConnection,
) -> QueryResult<Vec<RewardReconciliationMismatchRow>> {
    let candidates = reward_candidates::table
        .filter(
            reward_candidates::status
                .eq(REWARD_STATUS_TOKEN_CONFIRMED)
                .or(reward_candidates::status.eq(REWARD_STATUS_WALLET_CREDITED))
                .or(reward_candidates::status.eq(REWARD_STATUS_NOTIFIED))
                .or(reward_candidates::status.eq(REWARD_STATUS_COMPLETED))
                .or(reward_candidates::status.eq(REWARD_STATUS_NEEDS_RECONCILIATION)),
        )
        .order(reward_candidates::updated_at.desc())
        .load::<RewardCandidate>(conn)
        .await?;

    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    let candidate_ids = candidates
        .iter()
        .map(|candidate| candidate.id)
        .collect::<Vec<_>>();
    let payout_records = reward_payout_records::table
        .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids.as_slice()))
        .select(reward_payout_records::reward_candidate_id)
        .load::<i64>(conn)
        .await?
        .into_iter()
        .map(|candidate_id| (candidate_id, true))
        .collect::<HashMap<_, _>>();
    let credit_records = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq_any(candidate_ids.as_slice()))
        .select((
            reward_wallet_credit_records::reward_candidate_id,
            reward_wallet_credit_records::notification_id,
        ))
        .load::<(i64, Option<i64>)>(conn)
        .await?
        .into_iter()
        .collect::<HashMap<_, _>>();

    let mut mismatches = Vec::new();
    for candidate in candidates {
        let has_payout_record = payout_records.contains_key(&candidate.id);
        let credit_notification_id = credit_records.get(&candidate.id).copied();
        let mismatch_type = match candidate.status.as_str() {
            REWARD_STATUS_NEEDS_RECONCILIATION => Some("needs_reconciliation"),
            REWARD_STATUS_TOKEN_CONFIRMED if !has_payout_record => Some("needs_payout_record"),
            REWARD_STATUS_TOKEN_CONFIRMED if credit_notification_id.is_none() => {
                Some("needs_wallet_credit")
            }
            REWARD_STATUS_WALLET_CREDITED if credit_notification_id.is_none() => {
                Some("needs_wallet_credit_record")
            }
            REWARD_STATUS_WALLET_CREDITED if credit_notification_id.flatten().is_none() => {
                Some("needs_notification")
            }
            REWARD_STATUS_NOTIFIED | REWARD_STATUS_COMPLETED
                if credit_notification_id.flatten().is_none() =>
            {
                Some("needs_notification_record")
            }
            _ => None,
        };

        if let Some(mismatch_type) = mismatch_type {
            mismatches.push(RewardReconciliationMismatchRow {
                reward_candidate_id: candidate.id,
                course_id: candidate.course_id,
                student_user_id: candidate.student_user_id,
                status: candidate.status,
                mismatch_type: mismatch_type.to_string(),
                approved_amount: candidate.approved_amount.as_ref().map(ToString::to_string),
                updated_at: candidate.updated_at,
            });
        }
    }

    Ok(mismatches.into_iter().take(50).collect())
}

impl From<RewardCandidate> for RewardCandidateDashboardRow {
    fn from(candidate: RewardCandidate) -> Self {
        RewardCandidateDashboardRow {
            reward_candidate_id: candidate.id,
            course_id: candidate.course_id,
            student_user_id: candidate.student_user_id,
            submitter_user_id: candidate.submitter_user_id,
            source_organization_id: candidate.source_organization_id,
            event_type: candidate.event_type,
            status: candidate.status,
            approved_amount: candidate.approved_amount.as_ref().map(ToString::to_string),
            updated_at: candidate.updated_at,
        }
    }
}
