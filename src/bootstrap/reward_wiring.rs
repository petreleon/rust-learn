use std::sync::Arc;

use actix_web::web;

use crate::application::rewards::decide_amount::RewardAmountDecisionUseCase;
use crate::application::rewards::decide_teacher_candidate::TeacherRewardCandidateDecisionUseCase;
use crate::application::rewards::list_candidate_audit::RewardCandidateAuditUseCase;
use crate::application::rewards::list_course_candidates::CourseRewardCandidatesUseCase;
use crate::application::rewards::list_platform_candidates::PlatformRewardCandidatesUseCase;
use crate::application::rewards::list_reward_history::StudentRewardHistoryUseCase;
use crate::application::rewards::manage_fraud_block::RewardFraudBlockUseCase;
use crate::application::rewards::manage_reward_policy::RewardPolicyUseCase;
use crate::application::rewards::submit_candidate::RewardCandidateSubmissionUseCase;
use crate::infra::postgres::rewards::course_reward_candidate_use_case::PostgresCourseRewardCandidatesUseCase;
use crate::infra::postgres::rewards::platform_reward_candidate_use_case::PostgresPlatformRewardCandidatesUseCase;
use crate::infra::postgres::rewards::reward_amount_decision_use_case::PostgresRewardAmountDecisionUseCase;
use crate::infra::postgres::rewards::reward_candidate_audit_use_case::PostgresRewardCandidateAuditUseCase;
use crate::infra::postgres::rewards::reward_candidate_submission_use_case::PostgresRewardCandidateSubmissionUseCase;
use crate::infra::postgres::rewards::reward_fraud_block_use_case::PostgresRewardFraudBlockUseCase;
use crate::infra::postgres::rewards::reward_history_use_case::PostgresStudentRewardHistoryUseCase;
use crate::infra::postgres::rewards::reward_policy_use_case::PostgresRewardPolicyUseCase;
use crate::infra::postgres::rewards::teacher_reward_candidate_decision_use_case::PostgresTeacherRewardCandidateDecisionUseCase;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct RewardUseCases {
    pub amount_decision: Arc<dyn RewardAmountDecisionUseCase>,
    pub candidate_audit: Arc<dyn RewardCandidateAuditUseCase>,
    pub candidate_submission: Arc<dyn RewardCandidateSubmissionUseCase>,
    pub course_candidates: Arc<dyn CourseRewardCandidatesUseCase>,
    pub fraud_block: Arc<dyn RewardFraudBlockUseCase>,
    pub history: Arc<dyn StudentRewardHistoryUseCase>,
    pub platform_candidates: Arc<dyn PlatformRewardCandidatesUseCase>,
    pub policy: Arc<dyn RewardPolicyUseCase>,
    pub teacher_candidate_decision: Arc<dyn TeacherRewardCandidateDecisionUseCase>,
}

pub fn build_reward_use_cases(pool: &DbPool) -> RewardUseCases {
    RewardUseCases {
        amount_decision: Arc::new(PostgresRewardAmountDecisionUseCase::new(pool.clone())),
        candidate_audit: Arc::new(PostgresRewardCandidateAuditUseCase::new(pool.clone())),
        candidate_submission: Arc::new(PostgresRewardCandidateSubmissionUseCase::new(pool.clone())),
        course_candidates: Arc::new(PostgresCourseRewardCandidatesUseCase::new(pool.clone())),
        fraud_block: Arc::new(PostgresRewardFraudBlockUseCase::new(pool.clone())),
        history: Arc::new(PostgresStudentRewardHistoryUseCase::new(pool.clone())),
        platform_candidates: Arc::new(PostgresPlatformRewardCandidatesUseCase::new(pool.clone())),
        policy: Arc::new(PostgresRewardPolicyUseCase::new(pool.clone())),
        teacher_candidate_decision: Arc::new(PostgresTeacherRewardCandidateDecisionUseCase::new(
            pool.clone(),
        )),
    }
}

pub fn configure_reward_app_data(cfg: &mut web::ServiceConfig, use_cases: &RewardUseCases) {
    cfg.app_data(web::Data::new(use_cases.amount_decision.clone()))
        .app_data(web::Data::new(use_cases.candidate_audit.clone()))
        .app_data(web::Data::new(use_cases.candidate_submission.clone()))
        .app_data(web::Data::new(use_cases.course_candidates.clone()))
        .app_data(web::Data::new(use_cases.fraud_block.clone()))
        .app_data(web::Data::new(use_cases.history.clone()))
        .app_data(web::Data::new(use_cases.platform_candidates.clone()))
        .app_data(web::Data::new(use_cases.policy.clone()))
        .app_data(web::Data::new(use_cases.teacher_candidate_decision.clone()));
}
