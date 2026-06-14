use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses_organizations, reward_candidates, reward_policies};
use crate::domain::rewards::candidate::event_type::{
    REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT, REWARD_EVENT_ASSESSMENT_COMPLETION,
    REWARD_EVENT_COURSE_COMPLETION, REWARD_EVENT_MANUAL_COMPLETION,
};
use crate::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_REJECTED, REWARD_STATUS_FAILED, REWARD_STATUS_TEACHER_REJECTED,
};
use crate::domain::rewards::policy::{
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::repositories::course_repository::user_permission_course_request;
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::Value;

use super::ensure_exact_course_permission::ensure_exact_course_permission;
use super::support::RewardCandidateError;

pub(super) async fn ensure_active_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> Result<(), RewardCandidateError> {
    let has_course_policy = diesel::select(exists(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
            .filter(reward_policies::course_id.eq(Some(course_id)))
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true)),
    ))
    .get_result(conn)
    .await?;
    if has_course_policy {
        return Ok(());
    }

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;
    if !organization_ids.is_empty() {
        let has_organization_policy = diesel::select(exists(
            reward_policies::table
                .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
                .filter(reward_policies::organization_id.eq_any(organization_ids))
                .filter(reward_policies::course_id.is_null())
                .filter(reward_policies::event_type.eq(event_type))
                .filter(reward_policies::active.eq(true)),
        ))
        .get_result(conn)
        .await?;
        if has_organization_policy {
            return Ok(());
        }
    }

    let has_platform_policy = diesel::select(exists(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
            .filter(reward_policies::organization_id.is_null())
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true)),
    ))
    .get_result(conn)
    .await?;

    if has_platform_policy {
        Ok(())
    } else {
        Err(RewardCandidateError::InvalidInput(
            "no active reward policy covers this course event".to_string(),
        ))
    }
}

pub(super) async fn ensure_no_prior_active_reward_candidate(
    conn: &mut AsyncPgConnection,
    student_user_id: i32,
    course_id: i32,
    event_type: &str,
) -> Result<(), RewardCandidateError> {
    let already_exists = diesel::select(exists(
        reward_candidates::table
            .filter(reward_candidates::student_user_id.eq(student_user_id))
            .filter(reward_candidates::course_id.eq(course_id))
            .filter(reward_candidates::event_type.eq(event_type))
            .filter(reward_candidates::status.ne(REWARD_STATUS_TEACHER_REJECTED))
            .filter(reward_candidates::status.ne(REWARD_STATUS_AMOUNT_REJECTED))
            .filter(reward_candidates::status.ne(REWARD_STATUS_FAILED)),
    ))
    .get_result(conn)
    .await?;

    if already_exists {
        Err(RewardCandidateError::InvalidInput(
            "an active reward candidate already exists for this course event".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub(super) fn ensure_reward_evidence_is_eligible(
    event_type: &str,
    evidence: &Value,
) -> Result<(), RewardCandidateError> {
    match event_type {
        REWARD_EVENT_COURSE_COMPLETION => {
            ensure_evidence_number_at_least(evidence, "completion_percentage", 100.0)
        }
        REWARD_EVENT_ASSESSMENT_COMPLETION => {
            ensure_evidence_number_at_least(evidence, "passing_score", 70.0)
        }
        REWARD_EVENT_MANUAL_COMPLETION | REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT => Ok(()),
        _ => unreachable!("event type was normalized before evidence validation"),
    }
}

pub(super) fn ensure_evidence_number_at_least(
    evidence: &Value,
    key: &str,
    minimum: f64,
) -> Result<(), RewardCandidateError> {
    let value = evidence.get(key).and_then(Value::as_f64).ok_or_else(|| {
        RewardCandidateError::InvalidInput(format!("{} evidence is required", key))
    })?;

    if value >= minimum {
        Ok(())
    } else {
        Err(RewardCandidateError::InvalidInput(format!(
            "{} evidence is below the reward threshold",
            key
        )))
    }
}

pub(super) async fn ensure_course_submission_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), RewardCandidateError> {
    for permission in [
        Permissions::SUBMIT_COURSE_REWARD_EVENT,
        Permissions::CREATE_REWARDABLE_COURSE_EVENT,
    ] {
        if user_permission_course_request(conn, user_id, course_id, &permission.to_string()).await?
        {
            return Ok(());
        }
    }

    Err(RewardCandidateError::PermissionDenied(
        Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
    ))
}

pub(super) async fn ensure_course_teacher_approval_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), RewardCandidateError> {
    ensure_exact_course_permission(
        conn,
        user_id,
        course_id,
        Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
    )
    .await
}
