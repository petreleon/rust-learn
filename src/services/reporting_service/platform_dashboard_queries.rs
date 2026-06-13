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
        .filter(reward_execution_jobs::status.eq(RewardExecutionJobStatus::Failed.as_str()))
        .order(reward_execution_jobs::updated_at.desc())
        .limit(50)
        .load::<RewardExecutionJob>(conn)
        .await?
        .into_iter()
        .map(RewardExecutionFailureRow::from)
        .collect::<Vec<_>>();
    let payout_failure_count = reward_execution_jobs::table
        .filter(reward_execution_jobs::status.eq(RewardExecutionJobStatus::Failed.as_str()))
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

impl From<RewardExecutionJob> for RewardExecutionFailureRow {
    fn from(job: RewardExecutionJob) -> Self {
        RewardExecutionFailureRow {
            reward_execution_job_id: job.id,
            reward_candidate_id: job.reward_candidate_id,
            status: job.status,
            attempts: job.attempts,
            last_error: job.last_error,
            updated_at: job.updated_at,
        }
    }
}
