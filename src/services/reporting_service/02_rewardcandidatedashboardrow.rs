#[derive(Debug, Serialize)]
pub struct RewardCandidateDashboardRow {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_organization_id: Option<i32>,
    pub event_type: String,
    pub status: String,
    pub approved_amount: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct RewardExecutionFailureRow {
    pub reward_execution_job_id: i64,
    pub reward_candidate_id: i64,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct RewardReconciliationMismatchRow {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub status: String,
    pub mismatch_type: String,
    pub approved_amount: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

fn csv_value(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn csv_optional(value: Option<impl ToString>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

pub async fn platform_reward_dashboard(
    conn: &mut AsyncPgConnection,
) -> QueryResult<PlatformRewardDashboard> {
    let teacher_applications = teacher_application_dashboard_summary(conn).await?;
    let reward_candidates = reward_candidate_dashboard_summary(conn).await?;

    let pending_amount_approvals = reward_candidates::table
        .filter(reward_candidates::status.eq(REWARD_STATUS_TEACHER_APPROVED))
        .order(reward_candidates::updated_at.desc())
        .limit(50)
        .load::<RewardCandidate>(conn)
        .await?
        .into_iter()
        .map(RewardCandidateDashboardRow::from)
        .collect::<Vec<_>>();
    let pending_amount_approval_count = reward_candidates::table
        .filter(reward_candidates::status.eq(REWARD_STATUS_TEACHER_APPROVED))
        .count()
        .get_result(conn)
        .await?;

    let payout_failures = reward_execution_jobs::table
        .filter(reward_execution_jobs::status.eq("failed"))
        .order(reward_execution_jobs::updated_at.desc())
        .limit(50)
        .load::<RewardExecutionJob>(conn)
        .await?
        .into_iter()
        .map(RewardExecutionFailureRow::from)
        .collect::<Vec<_>>();
    let payout_failure_count = reward_execution_jobs::table
        .filter(reward_execution_jobs::status.eq("failed"))
        .count()
        .get_result(conn)
        .await?;

    let reconciliation_mismatches = reward_reconciliation_mismatches(conn).await?;
    let reconciliation_mismatch_count = reconciliation_mismatches.len() as i64;

    Ok(PlatformRewardDashboard {
        teacher_applications,
        reward_candidates,
        pending_amount_approval_count,
        pending_amount_approvals,
        payout_failure_count,
        payout_failures,
        reconciliation_mismatch_count,
        reconciliation_mismatches,
    })
}

pub async fn platform_fraud_dashboard(
    conn: &mut AsyncPgConnection,
) -> QueryResult<PlatformFraudDashboard> {
    let now = chrono::Utc::now();
    let active_blocks = reward_fraud_blocks::table
        .filter(reward_fraud_blocks::revoked_at.is_null())
        .filter(
            reward_fraud_blocks::expires_at
                .is_null()
                .or(reward_fraud_blocks::expires_at.gt(now)),
        )
        .order(reward_fraud_blocks::created_at.desc())
        .limit(100)
        .load::<RewardFraudBlock>(conn)
        .await?;

    let mut active_by_scope = FraudBlockScopeSummary::default();
    for block in &active_blocks {
        match block.scope_type.as_str() {
            REWARD_FRAUD_BLOCK_SCOPE_TEACHER => active_by_scope.teacher += 1,
            REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => active_by_scope.organization += 1,
            REWARD_FRAUD_BLOCK_SCOPE_COURSE => active_by_scope.course += 1,
            REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => active_by_scope.reward_policy += 1,
            _ => {}
        }
    }

    Ok(PlatformFraudDashboard {
        active_total: active_blocks.len() as i64,
        active_by_scope,
        active_blocks: active_blocks
            .into_iter()
            .map(FraudBlockDashboardRow::from)
            .collect(),
    })
}

pub async fn platform_report_summary(
    conn: &mut AsyncPgConnection,
) -> QueryResult<PlatformReportSummary> {
    Ok(PlatformReportSummary {
        total_users: users::table.count().get_result(conn).await?,
        total_organizations: organizations::table.count().get_result(conn).await?,
        total_courses: courses::table.count().get_result(conn).await?,
        total_wallets: wallets::table.count().get_result(conn).await?,
        total_notifications: notifications::table.count().get_result(conn).await?,
    })
}
