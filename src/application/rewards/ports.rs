use futures::future::BoxFuture;

use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditError, RewardCandidateAuditEvent,
};
use crate::application::rewards::list_course_candidates::{
    CourseRewardCandidate, CourseRewardCandidatesError, CourseRewardCandidatesFilter,
};
use crate::application::rewards::list_reward_history::{
    StudentRewardCandidateRecord, StudentRewardHistoryError, StudentRewardHistoryFilter,
    StudentRewardTokenTransaction, StudentRewardWalletCredit,
};
use crate::application::rewards::manage_fraud_block::{
    RewardFraudBlockDraft, RewardFraudBlockError, RewardFraudBlockListFilter,
    RewardFraudBlockOutput,
};
use crate::application::rewards::manage_reward_policy::{
    RewardPolicyDraft, RewardPolicyError, RewardPolicyListFilter, RewardPolicyOutput,
};

pub trait RewardPolicyStore {
    fn can_manage_reward_policies(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardPolicyError>>;

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<(), RewardPolicyError>>;

    fn course_exists(&mut self, course_id: i32) -> BoxFuture<'_, Result<(), RewardPolicyError>>;

    fn create_versioned_policy(
        &mut self,
        draft: RewardPolicyDraft,
    ) -> BoxFuture<'_, Result<RewardPolicyOutput, RewardPolicyError>>;

    fn list_policies(
        &mut self,
        filter: RewardPolicyListFilter,
    ) -> BoxFuture<'_, Result<Vec<RewardPolicyOutput>, RewardPolicyError>>;
}

pub trait RewardFraudBlockStore {
    fn can_manage_fraud_block_scope<'a>(
        &'a mut self,
        actor_user_id: i32,
        scope_type: &'a str,
    ) -> BoxFuture<'a, Result<bool, RewardFraudBlockError>>;

    fn can_view_fraud_blocks(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardFraudBlockError>>;

    fn create_fraud_block(
        &mut self,
        draft: RewardFraudBlockDraft,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>>;

    fn find_fraud_block(
        &mut self,
        block_id: i64,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>>;

    fn list_fraud_blocks(
        &mut self,
        filter: RewardFraudBlockListFilter,
    ) -> BoxFuture<'_, Result<(Vec<RewardFraudBlockOutput>, i64), RewardFraudBlockError>>;

    fn revoke_fraud_block(
        &mut self,
        block_id: i64,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>>;

    fn notify_fraud_block_transition<'a>(
        &'a mut self,
        block: &'a RewardFraudBlockOutput,
        event_type: &'a str,
    ) -> BoxFuture<'a, Result<(), RewardFraudBlockError>>;
}

pub trait StudentRewardHistoryStore {
    fn list_student_reward_candidates(
        &mut self,
        filter: StudentRewardHistoryFilter,
    ) -> BoxFuture<'_, Result<Vec<StudentRewardCandidateRecord>, StudentRewardHistoryError>>;

    fn can_view_course_reward_status(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, StudentRewardHistoryError>>;

    fn load_wallet_credit(
        &mut self,
        reward_candidate_id: i64,
    ) -> BoxFuture<'_, Result<Option<StudentRewardWalletCredit>, StudentRewardHistoryError>>;

    fn load_token_transaction(
        &mut self,
        reward_candidate_id: i64,
    ) -> BoxFuture<'_, Result<Option<StudentRewardTokenTransaction>, StudentRewardHistoryError>>;
}

pub trait RewardCandidateAuditStore {
    fn can_view_reward_audit(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateAuditError>>;

    fn reward_candidate_exists(
        &mut self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<(), RewardCandidateAuditError>>;

    fn list_candidate_audit_events(
        &mut self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardCandidateAuditEvent>, RewardCandidateAuditError>>;
}

pub trait CourseRewardCandidateStore {
    fn course_exists(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<(), CourseRewardCandidatesError>>;

    fn can_approve_student_reward_candidate(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>>;

    fn can_manage_course_reward_rules(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>>;

    fn can_view_course_reward_status(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>>;

    fn list_course_reward_candidates(
        &mut self,
        filter: CourseRewardCandidatesFilter,
    ) -> BoxFuture<'_, Result<Vec<CourseRewardCandidate>, CourseRewardCandidatesError>>;
}
