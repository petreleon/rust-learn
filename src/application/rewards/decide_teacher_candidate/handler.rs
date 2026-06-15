use crate::application::rewards::decide_teacher_candidate::validation::normalize_teacher_decision_status;
use crate::application::rewards::decide_teacher_candidate::{
    TeacherRewardCandidateDecision, TeacherRewardCandidateDecisionCommand,
    TeacherRewardCandidateDecisionError, TeacherRewardCandidateDecisionOutput,
    TeacherRewardCandidateDecisionStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn decide_teacher_reward_candidate(
    store: &mut impl TeacherRewardCandidateDecisionStore,
    actor_user_id: i32,
    course_id: i32,
    candidate_id: i64,
    command: TeacherRewardCandidateDecisionCommand,
) -> Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError> {
    let target_status = normalize_teacher_decision_status(&command.status)?;
    ensure_can_approve_reward_candidate(store, actor_user_id, course_id).await?;

    store
        .decide_teacher_reward_candidate(TeacherRewardCandidateDecision {
            actor_user_id,
            course_id,
            candidate_id,
            target_status,
            decision_reason: command.decision_reason,
        })
        .await
}

async fn ensure_can_approve_reward_candidate(
    store: &mut impl TeacherRewardCandidateDecisionStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<(), TeacherRewardCandidateDecisionError> {
    if store
        .can_approve_student_reward_candidate(actor_user_id, course_id)
        .await?
    {
        Ok(())
    } else {
        Err(TeacherRewardCandidateDecisionError::PermissionDenied(
            Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.into(),
        ))
    }
}

#[cfg(test)]
mod tests;
