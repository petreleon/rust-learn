use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::submit_candidate::RewardCandidateSubmissionError;
use crate::domain::rewards::candidate::lifecycle;
use crate::infra::postgres::rewards::reward_authorization_access;
use crate::infra::postgres::rewards::reward_candidate_policy_lookup::active_reward_policy_ids_for_course_event;
use crate::infra::postgres::rewards::reward_candidate_submission_mappers::map_reward_candidate_submission_error;
use crate::infra::postgres::schema::{reward_candidates, users};

pub(super) async fn ensure_reward_target_eligible(
    conn: &mut AsyncPgConnection,
    student_user_id: i32,
    course_id: i32,
    event_type: &str,
) -> Result<(), RewardCandidateSubmissionError> {
    let (_user_id, email_verified) = users::table
        .find(student_user_id)
        .select((users::id, users::email_verified))
        .first::<(i32, bool)>(conn)
        .await
        .map_err(map_reward_candidate_submission_error)?;

    if !email_verified {
        return Err(RewardCandidateSubmissionError::InvalidInput(
            "reward recipient must have a verified email".to_string(),
        ));
    }

    let can_view_reward_status = reward_authorization_access::can_view_course_reward_status(
        conn,
        student_user_id,
        course_id,
    )
    .await
    .map_err(|error| RewardCandidateSubmissionError::Database(error.to_string()))?;

    if !can_view_reward_status {
        return Err(RewardCandidateSubmissionError::InvalidInput(
            "reward recipient is not eligible for this course".to_string(),
        ));
    }

    ensure_active_reward_policy(conn, course_id, event_type).await
}

pub(super) async fn ensure_no_prior_active_reward_candidate(
    conn: &mut AsyncPgConnection,
    student_user_id: i32,
    course_id: i32,
    event_type: &str,
) -> Result<(), RewardCandidateSubmissionError> {
    let reusable_statuses = lifecycle::prior_candidate_statuses_allowing_new_submission();
    let already_exists = diesel::select(exists(
        reward_candidates::table
            .filter(reward_candidates::student_user_id.eq(student_user_id))
            .filter(reward_candidates::course_id.eq(course_id))
            .filter(reward_candidates::event_type.eq(event_type))
            .filter(reward_candidates::status.ne(reusable_statuses[0].as_str()))
            .filter(reward_candidates::status.ne(reusable_statuses[1].as_str()))
            .filter(reward_candidates::status.ne(reusable_statuses[2].as_str())),
    ))
    .get_result(conn)
    .await
    .map_err(map_reward_candidate_submission_error)?;

    if already_exists {
        Err(RewardCandidateSubmissionError::InvalidInput(
            "an active reward candidate already exists for this course event".to_string(),
        ))
    } else {
        Ok(())
    }
}

async fn ensure_active_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> Result<(), RewardCandidateSubmissionError> {
    let policy_ids = active_reward_policy_ids_for_course_event(conn, course_id, event_type)
        .await
        .map_err(map_reward_candidate_submission_error)?;
    if policy_ids.is_empty() {
        Err(RewardCandidateSubmissionError::InvalidInput(
            "no active reward policy covers this course event".to_string(),
        ))
    } else {
        Ok(())
    }
}
