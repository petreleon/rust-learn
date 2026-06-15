use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmission, RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
    RewardCandidateSubmissionStore, SubmitRewardCandidateCommand,
};
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;

pub async fn submit_course_reward_candidate(
    store: &mut impl RewardCandidateSubmissionStore,
    actor_user_id: i32,
    course_id: i32,
    command: SubmitRewardCandidateCommand,
) -> Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError> {
    store.course_exists(course_id).await?;
    ensure_can_submit_course_reward_event(store, actor_user_id, course_id).await?;
    store
        .submit_reward_candidate(RewardCandidateSubmission {
            actor_user_id,
            course_id,
            source_scope: RewardCandidateSourceScope::Course,
            source_organization_id: None,
            command,
        })
        .await
}

pub async fn submit_organization_reward_candidate(
    store: &mut impl RewardCandidateSubmissionStore,
    actor_user_id: i32,
    organization_id: i32,
    course_id: i32,
    command: SubmitRewardCandidateCommand,
) -> Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError> {
    store.course_exists(course_id).await?;
    ensure_course_attached_to_organization(store, course_id, organization_id).await?;
    ensure_can_submit_organization_reward_event(store, actor_user_id, organization_id).await?;
    store
        .submit_reward_candidate(RewardCandidateSubmission {
            actor_user_id,
            course_id,
            source_scope: RewardCandidateSourceScope::Organization,
            source_organization_id: Some(organization_id),
            command,
        })
        .await
}

async fn ensure_can_submit_course_reward_event(
    store: &mut impl RewardCandidateSubmissionStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<(), RewardCandidateSubmissionError> {
    if store
        .can_submit_course_reward_event(actor_user_id, course_id)
        .await?
    {
        Ok(())
    } else {
        Err(RewardCandidateSubmissionError::PermissionDenied(
            "SUBMIT_COURSE_REWARD_EVENT".to_string(),
        ))
    }
}

async fn ensure_course_attached_to_organization(
    store: &mut impl RewardCandidateSubmissionStore,
    course_id: i32,
    organization_id: i32,
) -> Result<(), RewardCandidateSubmissionError> {
    if store
        .course_attached_to_organization(course_id, organization_id)
        .await?
    {
        Ok(())
    } else {
        Err(RewardCandidateSubmissionError::InvalidInput(
            "course is not attached to the organization".to_string(),
        ))
    }
}

async fn ensure_can_submit_organization_reward_event(
    store: &mut impl RewardCandidateSubmissionStore,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<(), RewardCandidateSubmissionError> {
    if store
        .can_submit_organization_course_reward_event(actor_user_id, organization_id)
        .await?
    {
        Ok(())
    } else {
        Err(RewardCandidateSubmissionError::PermissionDenied(
            "SUBMIT_ORG_COURSE_REWARD_EVENT".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests;
