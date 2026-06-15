use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_csv_exports::{
    platform_reward_approval_export_row, PlatformCsvExportError, PlatformRewardApprovalExportFact,
    PlatformRewardApprovalExportRowOutput,
};
use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::models::reward_candidate::RewardCandidate;
use crate::infra::postgres::reporting::platform_csv_export_mappers::map_diesel_error;
use crate::infra::postgres::schema::reward_candidates;

pub(super) async fn load_reward_approval_export_rows(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<PlatformRewardApprovalExportRowOutput>, PlatformCsvExportError> {
    reward_candidates::table
        .filter(
            reward_candidates::teacher_decided_at
                .is_not_null()
                .or(reward_candidates::amount_decided_at.is_not_null()),
        )
        .order(reward_candidates::updated_at.desc())
        .limit(1000)
        .load::<RewardCandidate>(conn)
        .await
        .map_err(map_diesel_error)?
        .into_iter()
        .map(reward_approval_fact)
        .map(|fact| fact.map(platform_reward_approval_export_row))
        .collect()
}

fn reward_approval_fact(
    candidate: RewardCandidate,
) -> Result<PlatformRewardApprovalExportFact, PlatformCsvExportError> {
    let source_scope = RewardCandidateSourceScope::parse(&candidate.source_scope)
        .map_err(|error| PlatformCsvExportError::Database(error.to_string()))?;
    let event_type = RewardEventType::parse(&candidate.event_type)
        .map_err(|error| PlatformCsvExportError::Database(error.to_string()))?;
    let status = RewardCandidateStatus::parse(&candidate.status)
        .map_err(|error| PlatformCsvExportError::Database(error.to_string()))?;

    Ok(PlatformRewardApprovalExportFact {
        reward_candidate_id: candidate.id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        submitter_user_id: candidate.submitter_user_id,
        source_scope,
        source_organization_id: candidate.source_organization_id,
        event_type,
        status,
        teacher_approver_user_id: candidate.teacher_approver_user_id,
        teacher_decision_reason: candidate.teacher_decision_reason,
        teacher_decided_at: candidate.teacher_decided_at,
        amount_reviewer_user_id: candidate.amount_reviewer_user_id,
        approved_amount: candidate.approved_amount,
        amount_decision_reason: candidate.amount_decision_reason,
        amount_decided_at: candidate.amount_decided_at,
        created_at: candidate.created_at,
        updated_at: candidate.updated_at,
    })
}
