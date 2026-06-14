use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_csv_exports::{
    PlatformCsvExportError, PlatformRewardApprovalExportRowOutput,
};
use crate::db::schema::reward_candidates;
use crate::infra::postgres::reporting::platform_csv_export_mappers::map_diesel_error;
use crate::models::reward_candidate::RewardCandidate;

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
        .map(|rows| rows.into_iter().map(map_reward_candidate).collect())
        .map_err(map_diesel_error)
}

fn map_reward_candidate(candidate: RewardCandidate) -> PlatformRewardApprovalExportRowOutput {
    PlatformRewardApprovalExportRowOutput {
        reward_candidate_id: candidate.id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        submitter_user_id: candidate.submitter_user_id,
        source_scope: candidate.source_scope,
        source_organization_id: candidate.source_organization_id,
        event_type: candidate.event_type,
        status: candidate.status,
        teacher_approver_user_id: candidate.teacher_approver_user_id,
        teacher_decision_reason: candidate.teacher_decision_reason.unwrap_or_default(),
        teacher_decided_at: candidate.teacher_decided_at,
        amount_reviewer_user_id: candidate.amount_reviewer_user_id,
        approved_amount: candidate
            .approved_amount
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
        amount_decision_reason: candidate.amount_decision_reason.unwrap_or_default(),
        amount_decided_at: candidate.amount_decided_at,
        created_at: candidate.created_at,
        updated_at: candidate.updated_at,
    }
}
