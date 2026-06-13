use std::sync::Arc;

use actix_web::web;
use chrono::Utc;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::rewards::submit_candidate::{
    RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
    RewardCandidateSubmissionUseCase, SubmitRewardCandidateCommand,
};
use serde_json::json;

struct RouteOnlyRewardCandidateSubmissionUseCase;

pub fn reward_candidate_submission_data() -> web::Data<Arc<dyn RewardCandidateSubmissionUseCase>> {
    web::Data::new(Arc::new(RouteOnlyRewardCandidateSubmissionUseCase)
        as Arc<dyn RewardCandidateSubmissionUseCase>)
}

impl RewardCandidateSubmissionUseCase for RouteOnlyRewardCandidateSubmissionUseCase {
    fn submit_course_reward_candidate(
        &self,
        _actor_user_id: i32,
        course_id: i32,
        command: SubmitRewardCandidateCommand,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>
    {
        ready(Ok(submission_output(course_id, None, command))).boxed()
    }

    fn submit_organization_reward_candidate(
        &self,
        _actor_user_id: i32,
        organization_id: i32,
        course_id: i32,
        command: SubmitRewardCandidateCommand,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>
    {
        ready(Ok(submission_output(
            course_id,
            Some(organization_id),
            command,
        )))
        .boxed()
    }
}

fn submission_output(
    course_id: i32,
    source_organization_id: Option<i32>,
    command: SubmitRewardCandidateCommand,
) -> RewardCandidateSubmissionOutput {
    let now = Utc::now();
    RewardCandidateSubmissionOutput {
        id: 34,
        course_id,
        student_user_id: command.student_user_id,
        submitter_user_id: 7,
        source_scope: if source_organization_id.is_some() {
            "organization"
        } else {
            "course"
        }
        .to_string(),
        source_organization_id,
        event_type: command.event_type,
        idempotency_key: command
            .idempotency_key
            .unwrap_or_else(|| "manual:12:23".to_string()),
        evidence: command.evidence.unwrap_or_else(|| json!({})),
        status: "pending_teacher_approval".to_string(),
        teacher_approver_user_id: None,
        teacher_decision_reason: None,
        teacher_decided_at: None,
        amount_reviewer_user_id: None,
        approved_amount: None,
        amount_decision_reason: None,
        amount_decided_at: None,
        created_at: now,
        updated_at: now,
    }
}
