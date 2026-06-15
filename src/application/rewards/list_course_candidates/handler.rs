use crate::application::rewards::list_course_candidates::{
    CourseRewardCandidate, CourseRewardCandidatesError, CourseRewardCandidatesFilter,
    CourseRewardCandidatesQuery,
};
use crate::application::rewards::ports::CourseRewardCandidateStore;
use crate::domain::access_control::permissions::Permissions;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub async fn list_course_reward_candidates(
    store: &mut impl CourseRewardCandidateStore,
    actor_user_id: i32,
    course_id: i32,
    query: CourseRewardCandidatesQuery,
) -> Result<Vec<CourseRewardCandidate>, CourseRewardCandidatesError> {
    store.course_exists(course_id).await?;
    let can_manage = can_manage_course_reward_candidates(store, actor_user_id, course_id).await?;
    if !can_manage {
        ensure_can_view_own_reward_status(store, actor_user_id, course_id).await?;
    }

    store
        .list_course_reward_candidates(CourseRewardCandidatesFilter {
            course_id,
            student_user_id: visible_student_user_id(
                can_manage,
                actor_user_id,
                query.student_user_id,
            ),
            status: normalized_status(query.status)?,
            limit: query.limit,
            offset: query.offset,
        })
        .await
}

async fn can_manage_course_reward_candidates(
    store: &mut impl CourseRewardCandidateStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, CourseRewardCandidatesError> {
    Ok(store
        .can_approve_student_reward_candidate(actor_user_id, course_id)
        .await?
        || store
            .can_manage_course_reward_rules(actor_user_id, course_id)
            .await?)
}

async fn ensure_can_view_own_reward_status(
    store: &mut impl CourseRewardCandidateStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<(), CourseRewardCandidatesError> {
    if store
        .can_view_course_reward_status(actor_user_id, course_id)
        .await?
    {
        Ok(())
    } else {
        Err(CourseRewardCandidatesError::PermissionDenied(
            Permissions::VIEW_COURSE_REWARD_STATUS.into(),
        ))
    }
}

fn visible_student_user_id(
    can_manage: bool,
    actor_user_id: i32,
    requested_student_user_id: Option<i32>,
) -> Option<i32> {
    if can_manage {
        requested_student_user_id
    } else {
        Some(actor_user_id)
    }
}

fn normalized_status(
    status: Option<String>,
) -> Result<Option<String>, CourseRewardCandidatesError> {
    status
        .map(|status| {
            RewardCandidateStatus::normalize(&status).map_err(|_| {
                CourseRewardCandidatesError::InvalidStatus(
                    "unsupported reward candidate status".to_string(),
                )
            })
        })
        .transpose()
}

#[cfg(test)]
mod tests;
