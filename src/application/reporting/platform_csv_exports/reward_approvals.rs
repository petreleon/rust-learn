use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::application::reporting::platform_csv_exports::PlatformRewardApprovalExportRowOutput;
use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PlatformRewardApprovalExportFact {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: RewardCandidateSourceScope,
    pub source_organization_id: Option<i32>,
    pub event_type: RewardEventType,
    pub status: RewardCandidateStatus,
    pub teacher_approver_user_id: Option<i32>,
    pub teacher_decision_reason: Option<String>,
    pub teacher_decided_at: Option<DateTime<Utc>>,
    pub amount_reviewer_user_id: Option<i32>,
    pub approved_amount: Option<BigDecimal>,
    pub amount_decision_reason: Option<String>,
    pub amount_decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) fn platform_reward_approval_export_row(
    fact: PlatformRewardApprovalExportFact,
) -> PlatformRewardApprovalExportRowOutput {
    PlatformRewardApprovalExportRowOutput {
        reward_candidate_id: fact.reward_candidate_id,
        course_id: fact.course_id,
        student_user_id: fact.student_user_id,
        submitter_user_id: fact.submitter_user_id,
        source_scope: fact.source_scope,
        source_organization_id: fact.source_organization_id,
        event_type: fact.event_type,
        status: fact.status,
        teacher_approver_user_id: fact.teacher_approver_user_id,
        teacher_decision_reason: fact.teacher_decision_reason.unwrap_or_default(),
        teacher_decided_at: fact.teacher_decided_at,
        amount_reviewer_user_id: fact.amount_reviewer_user_id,
        approved_amount: fact
            .approved_amount
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
        amount_decision_reason: fact.amount_decision_reason.unwrap_or_default(),
        amount_decided_at: fact.amount_decided_at,
        created_at: fact.created_at,
        updated_at: fact.updated_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_reward_approval_export_row_from_fact() {
        let now = Utc::now();

        let row = platform_reward_approval_export_row(PlatformRewardApprovalExportFact {
            reward_candidate_id: 42,
            course_id: 7,
            student_user_id: 8,
            submitter_user_id: 9,
            source_scope: RewardCandidateSourceScope::Course,
            source_organization_id: Some(10),
            event_type: RewardEventType::CourseCompletion,
            status: RewardCandidateStatus::AmountApproved,
            teacher_approver_user_id: Some(11),
            teacher_decision_reason: Some("teacher ok".to_string()),
            teacher_decided_at: Some(now),
            amount_reviewer_user_id: Some(12),
            approved_amount: Some(BigDecimal::from(25)),
            amount_decision_reason: Some("amount ok".to_string()),
            amount_decided_at: Some(now),
            created_at: now,
            updated_at: now,
        });

        assert_eq!(row.reward_candidate_id, 42);
        assert_eq!(row.source_scope, RewardCandidateSourceScope::Course);
        assert_eq!(row.status, RewardCandidateStatus::AmountApproved);
        assert_eq!(row.teacher_decision_reason, "teacher ok");
        assert_eq!(row.approved_amount, "25");
        assert_eq!(row.amount_decision_reason, "amount ok");
        assert_eq!(row.amount_decided_at, Some(now));
    }

    #[test]
    fn defaults_missing_optional_text_and_amount() {
        let now = Utc::now();

        let row = platform_reward_approval_export_row(PlatformRewardApprovalExportFact {
            reward_candidate_id: 1,
            course_id: 2,
            student_user_id: 3,
            submitter_user_id: 4,
            source_scope: RewardCandidateSourceScope::Course,
            source_organization_id: None,
            event_type: RewardEventType::CourseCompletion,
            status: RewardCandidateStatus::TeacherApproved,
            teacher_approver_user_id: None,
            teacher_decision_reason: None,
            teacher_decided_at: None,
            amount_reviewer_user_id: None,
            approved_amount: None,
            amount_decision_reason: None,
            amount_decided_at: None,
            created_at: now,
            updated_at: now,
        });

        assert_eq!(row.teacher_decision_reason, "");
        assert_eq!(row.approved_amount, "");
        assert_eq!(row.amount_decision_reason, "");
    }
}
