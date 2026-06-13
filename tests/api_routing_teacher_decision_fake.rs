use std::sync::Arc;

use actix_web::web;
use chrono::Utc;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::rewards::decide_teacher_candidate::{
    TeacherRewardCandidateDecisionCommand, TeacherRewardCandidateDecisionError,
    TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionUseCase,
};
use serde_json::json;

struct RouteOnlyTeacherRewardCandidateDecisionUseCase;

pub fn teacher_reward_candidate_decision_data(
) -> web::Data<Arc<dyn TeacherRewardCandidateDecisionUseCase>> {
    web::Data::new(Arc::new(RouteOnlyTeacherRewardCandidateDecisionUseCase)
        as Arc<dyn TeacherRewardCandidateDecisionUseCase>)
}

impl TeacherRewardCandidateDecisionUseCase for RouteOnlyTeacherRewardCandidateDecisionUseCase {
    fn decide_teacher_reward_candidate(
        &self,
        _actor_user_id: i32,
        _course_id: i32,
        candidate_id: i64,
        _command: TeacherRewardCandidateDecisionCommand,
    ) -> BoxFuture<
        '_,
        Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError>,
    > {
        ready(Ok(teacher_decision_output(candidate_id))).boxed()
    }
}

fn teacher_decision_output(candidate_id: i64) -> TeacherRewardCandidateDecisionOutput {
    let now = Utc::now();
    TeacherRewardCandidateDecisionOutput {
        id: candidate_id,
        course_id: 12,
        student_user_id: 23,
        submitter_user_id: 7,
        source_scope: "course".to_string(),
        source_organization_id: None,
        event_type: "manual_completion".to_string(),
        idempotency_key: "manual:12:23".to_string(),
        evidence: json!({}),
        status: "teacher_approved".to_string(),
        teacher_approver_user_id: Some(7),
        teacher_decision_reason: Some("route smoke".to_string()),
        teacher_decided_at: Some(now),
        amount_reviewer_user_id: None,
        approved_amount: None,
        amount_decision_reason: None,
        amount_decided_at: None,
        created_at: now,
        updated_at: now,
    }
}
