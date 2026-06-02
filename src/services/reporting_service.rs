use crate::db::schema::{
    courses, courses_organizations, notifications, organizations, reward_candidates,
    reward_execution_jobs, reward_fraud_blocks, reward_payout_records,
    reward_wallet_credit_records, teacher_applications, user_role_course, user_role_organization,
    users, wallets,
};
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED,
    REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION,
    REWARD_STATUS_NOTIFIED, REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::reward_execution_job::RewardExecutionJob;
use crate::models::reward_fraud_block::{
    RewardFraudBlock, REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use crate::models::teacher_application::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct PlatformReportSummary {
    pub total_users: i64,
    pub total_organizations: i64,
    pub total_courses: i64,
    pub total_wallets: i64,
    pub total_notifications: i64,
}

#[derive(Debug, Serialize)]
pub struct OrganizationReportSummary {
    pub organization_id: i32,
    pub organization_name: String,
    pub course_count: i64,
    pub member_count: i64,
    pub wallet_count: i64,
    pub course_role_assignment_count: i64,
}

#[derive(Debug, Serialize)]
pub struct PlatformRewardDashboard {
    pub teacher_applications: TeacherApplicationDashboardSummary,
    pub reward_candidates: RewardCandidateDashboardSummary,
    pub pending_amount_approval_count: i64,
    pub pending_amount_approvals: Vec<RewardCandidateDashboardRow>,
    pub payout_failure_count: i64,
    pub payout_failures: Vec<RewardExecutionFailureRow>,
    pub reconciliation_mismatch_count: i64,
    pub reconciliation_mismatches: Vec<RewardReconciliationMismatchRow>,
}

#[derive(Debug, Serialize)]
pub struct PlatformFraudDashboard {
    pub active_total: i64,
    pub active_by_scope: FraudBlockScopeSummary,
    pub active_blocks: Vec<FraudBlockDashboardRow>,
}

#[derive(Debug, Default, Serialize)]
pub struct FraudBlockScopeSummary {
    pub teacher: i64,
    pub organization: i64,
    pub course: i64,
    pub reward_policy: i64,
}

#[derive(Debug, Serialize)]
pub struct FraudBlockDashboardRow {
    pub id: i64,
    pub scope_type: String,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub created_by_user_id: i32,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default, Serialize)]
pub struct TeacherApplicationDashboardSummary {
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Default, Serialize)]
pub struct RewardCandidateDashboardSummary {
    pub total: i64,
    pub pending_teacher_approval: i64,
    pub teacher_approved: i64,
    pub teacher_rejected: i64,
    pub amount_approved: i64,
    pub amount_rejected: i64,
    pub token_pending: i64,
    pub token_confirmed: i64,
    pub wallet_credited: i64,
    pub notified: i64,
    pub completed: i64,
    pub needs_reconciliation: i64,
    pub failed: i64,
}

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

pub async fn platform_reward_dashboard(
    conn: &mut AsyncPgConnection,
) -> QueryResult<PlatformRewardDashboard> {
    let teacher_applications = teacher_application_dashboard_summary(conn).await?;
    let reward_candidates = reward_candidate_dashboard_summary(conn).await?;

    let pending_amount_approvals = reward_candidates::table
        .filter(reward_candidates::status.eq(REWARD_STATUS_TEACHER_APPROVED))
        .order(reward_candidates::updated_at.asc())
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

impl From<RewardFraudBlock> for FraudBlockDashboardRow {
    fn from(block: RewardFraudBlock) -> Self {
        FraudBlockDashboardRow {
            id: block.id,
            scope_type: block.scope_type,
            teacher_user_id: block.teacher_user_id,
            organization_id: block.organization_id,
            course_id: block.course_id,
            reward_policy_id: block.reward_policy_id,
            reason: block.reason,
            evidence_reference: block.evidence_reference,
            created_by_user_id: block.created_by_user_id,
            expires_at: block.expires_at,
            created_at: block.created_at,
            updated_at: block.updated_at,
        }
    }
}

pub async fn organization_report_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<OrganizationReportSummary> {
    let organization_name = organizations::table
        .find(organization_id)
        .select(organizations::name)
        .first::<String>(conn)
        .await?;

    let course_ids = courses_organizations::table
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await?;
    let course_count = course_ids.len() as i64;

    let member_ids = user_role_organization::table
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(user_role_organization::user_id)
        .distinct()
        .load::<Option<i32>>(conn)
        .await?;
    let member_count = member_ids.into_iter().flatten().count() as i64;

    let wallet_count = wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .count()
        .get_result(conn)
        .await?;

    let course_role_assignment_count = if course_ids.is_empty() {
        0
    } else {
        user_role_course::table
            .filter(user_role_course::course_id.eq_any(course_ids))
            .count()
            .get_result(conn)
            .await?
    };

    Ok(OrganizationReportSummary {
        organization_id,
        organization_name,
        course_count,
        member_count,
        wallet_count,
        course_role_assignment_count,
    })
}

pub fn platform_report_csv(summary: &PlatformReportSummary) -> String {
    format!(
        "metric,value\nusers,{}\norganizations,{}\ncourses,{}\nwallets,{}\nnotifications,{}\n",
        summary.total_users,
        summary.total_organizations,
        summary.total_courses,
        summary.total_wallets,
        summary.total_notifications
    )
}

pub fn platform_reward_dashboard_csv(dashboard: &PlatformRewardDashboard) -> String {
    let mut csv = String::from("section,metric,value\n");
    csv.push_str(&format!(
        "teacher_applications,total,{}\n",
        dashboard.teacher_applications.total
    ));
    csv.push_str(&format!(
        "teacher_applications,submitted,{}\n",
        dashboard.teacher_applications.submitted
    ));
    csv.push_str(&format!(
        "teacher_applications,needs_changes,{}\n",
        dashboard.teacher_applications.needs_changes
    ));
    csv.push_str(&format!(
        "teacher_applications,approved,{}\n",
        dashboard.teacher_applications.approved
    ));
    csv.push_str(&format!(
        "teacher_applications,rejected,{}\n",
        dashboard.teacher_applications.rejected
    ));
    csv.push_str(&format!(
        "reward_candidates,total,{}\n",
        dashboard.reward_candidates.total
    ));
    csv.push_str(&format!(
        "reward_candidates,pending_amount_approval,{}\n",
        dashboard.pending_amount_approval_count
    ));
    csv.push_str(&format!(
        "reward_candidates,payout_failures,{}\n",
        dashboard.payout_failure_count
    ));
    csv.push_str(&format!(
        "reward_candidates,reconciliation_mismatches,{}\n",
        dashboard.reconciliation_mismatch_count
    ));

    csv.push_str("\npending_amount_approvals,reward_candidate_id,course_id,student_user_id,event_type,status,approved_amount,updated_at\n");
    for row in &dashboard.pending_amount_approvals {
        csv.push_str(&format!(
            "pending_amount_approvals,{},{},{},{},{},{},{}\n",
            row.reward_candidate_id,
            row.course_id,
            row.student_user_id,
            csv_value(&row.event_type),
            csv_value(&row.status),
            csv_value(row.approved_amount.as_deref().unwrap_or("")),
            row.updated_at
        ));
    }

    csv.push_str("\npayout_failures,reward_execution_job_id,reward_candidate_id,status,attempts,last_error,updated_at\n");
    for row in &dashboard.payout_failures {
        csv.push_str(&format!(
            "payout_failures,{},{},{},{},{},{}\n",
            row.reward_execution_job_id,
            row.reward_candidate_id,
            csv_value(&row.status),
            row.attempts,
            csv_value(row.last_error.as_deref().unwrap_or("")),
            row.updated_at
        ));
    }

    csv.push_str("\nreconciliation_mismatches,reward_candidate_id,course_id,student_user_id,status,mismatch_type,approved_amount,updated_at\n");
    for row in &dashboard.reconciliation_mismatches {
        csv.push_str(&format!(
            "reconciliation_mismatches,{},{},{},{},{},{},{}\n",
            row.reward_candidate_id,
            row.course_id,
            row.student_user_id,
            csv_value(&row.status),
            csv_value(&row.mismatch_type),
            csv_value(row.approved_amount.as_deref().unwrap_or("")),
            row.updated_at
        ));
    }

    csv
}

pub fn platform_fraud_dashboard_csv(dashboard: &PlatformFraudDashboard) -> String {
    let mut csv = String::from("section,metric,value\n");
    csv.push_str(&format!(
        "fraud_blocks,active_total,{}\n",
        dashboard.active_total
    ));
    csv.push_str(&format!(
        "fraud_blocks,teacher,{}\n",
        dashboard.active_by_scope.teacher
    ));
    csv.push_str(&format!(
        "fraud_blocks,organization,{}\n",
        dashboard.active_by_scope.organization
    ));
    csv.push_str(&format!(
        "fraud_blocks,course,{}\n",
        dashboard.active_by_scope.course
    ));
    csv.push_str(&format!(
        "fraud_blocks,reward_policy,{}\n",
        dashboard.active_by_scope.reward_policy
    ));

    csv.push_str("\nactive_fraud_blocks,id,scope_type,teacher_user_id,organization_id,course_id,reward_policy_id,reason,evidence_reference,created_by_user_id,expires_at,created_at\n");
    for row in &dashboard.active_blocks {
        csv.push_str(&format!(
            "active_fraud_blocks,{},{},{},{},{},{},{},{},{},{},{}\n",
            row.id,
            csv_value(&row.scope_type),
            row.teacher_user_id
                .map(|value| value.to_string())
                .unwrap_or_default(),
            row.organization_id
                .map(|value| value.to_string())
                .unwrap_or_default(),
            row.course_id
                .map(|value| value.to_string())
                .unwrap_or_default(),
            row.reward_policy_id
                .map(|value| value.to_string())
                .unwrap_or_default(),
            csv_value(&row.reason),
            csv_value(row.evidence_reference.as_deref().unwrap_or("")),
            row.created_by_user_id,
            row.expires_at
                .map(|value| value.to_string())
                .unwrap_or_default(),
            row.created_at
        ));
    }

    csv
}

pub fn organization_report_csv(summary: &OrganizationReportSummary) -> String {
    format!(
        "metric,value\norganization_id,{}\norganization_name,{}\ncourses,{}\nmembers,{}\nwallets,{}\ncourse_role_assignments,{}\n",
        summary.organization_id,
        csv_value(&summary.organization_name),
        summary.course_count,
        summary.member_count,
        summary.wallet_count,
        summary.course_role_assignment_count
    )
}
