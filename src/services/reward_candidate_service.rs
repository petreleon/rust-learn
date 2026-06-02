use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, courses_organizations, reward_candidates, reward_fraud_blocks, reward_policies, users,
};
use crate::models::reward_candidate::{
    NewRewardCandidate, RewardCandidate, REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT,
    REWARD_EVENT_ASSESSMENT_COMPLETION, REWARD_EVENT_COURSE_COMPLETION,
    REWARD_EVENT_MANUAL_COMPLETION, REWARD_SOURCE_COURSE, REWARD_SOURCE_ORGANIZATION,
    REWARD_STATUS_ADJUSTED, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED,
    REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION,
    REWARD_STATUS_NOTIFIED, REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::reward_fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use crate::models::reward_policy::{
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::repositories::course_repository::user_permission_course_request;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_candidate_repository::{self, RewardCandidateFilter};
use crate::repositories::reward_execution_job_repository;
use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitRewardCandidateRequest {
    pub student_user_id: i32,
    pub event_type: String,
    pub idempotency_key: Option<String>,
    pub evidence: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TeacherRewardCandidateDecisionRequest {
    pub status: String,
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RewardAmountDecisionRequest {
    pub status: String,
    pub approved_amount: Option<BigDecimal>,
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListRewardCandidatesRequest {
    pub status: Option<String>,
    pub student_user_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RewardCandidateError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidStatus(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for RewardCandidateError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardCandidateError::NotFound,
            other => RewardCandidateError::Database(other.to_string()),
        }
    }
}

pub async fn submit_course_reward_candidate(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    request: SubmitRewardCandidateRequest,
) -> Result<RewardCandidate, RewardCandidateError> {
    ensure_course_exists(conn, course_id).await?;
    ensure_course_submission_permission(conn, actor_user_id, course_id).await?;
    create_reward_candidate(
        conn,
        actor_user_id,
        course_id,
        None,
        REWARD_SOURCE_COURSE,
        request,
    )
    .await
}

pub async fn submit_organization_reward_candidate(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    course_id: i32,
    request: SubmitRewardCandidateRequest,
) -> Result<RewardCandidate, RewardCandidateError> {
    ensure_course_exists(conn, course_id).await?;
    ensure_course_attached_to_organization(conn, course_id, organization_id).await?;
    ensure_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT,
    )
    .await?;
    create_reward_candidate(
        conn,
        actor_user_id,
        course_id,
        Some(organization_id),
        REWARD_SOURCE_ORGANIZATION,
        request,
    )
    .await
}

pub async fn decide_reward_candidate_by_teacher(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    candidate_id: i64,
    request: TeacherRewardCandidateDecisionRequest,
) -> Result<RewardCandidate, RewardCandidateError> {
    let target_status = normalize_teacher_decision_status(&request.status)?;
    ensure_course_teacher_approval_permission(conn, actor_user_id, course_id).await?;

    conn.transaction::<_, RewardCandidateError, _>(|conn| {
        Box::pin(async move {
            let existing = reward_candidate_repository::find_candidate(conn, candidate_id).await?;
            if existing.course_id != course_id {
                return Err(RewardCandidateError::NotFound);
            }

            if existing.status == target_status {
                return Ok(existing);
            }

            if existing.status != REWARD_STATUS_PENDING_TEACHER_APPROVAL {
                return Err(RewardCandidateError::InvalidStatus(
                    "reward candidate has already left teacher approval".to_string(),
                ));
            }

            ensure_no_active_reward_fraud_block(
                conn,
                &[actor_user_id],
                existing.course_id,
                &existing.event_type,
                existing.source_organization_id,
            )
            .await?;

            reward_candidate_repository::update_teacher_decision(
                conn,
                candidate_id,
                actor_user_id,
                &target_status,
                request.decision_reason.as_deref(),
                Utc::now(),
            )
            .await
            .map_err(RewardCandidateError::from)
        })
    })
    .await
}

pub async fn decide_reward_amount(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
    request: RewardAmountDecisionRequest,
) -> Result<RewardCandidate, RewardCandidateError> {
    let target_status = normalize_amount_decision_status(&request.status)?;
    ensure_platform_permission(conn, actor_user_id, Permissions::APPROVE_REWARD_AMOUNT).await?;

    let approved_amount = match target_status.as_str() {
        REWARD_STATUS_AMOUNT_APPROVED => {
            let amount = request.approved_amount.ok_or_else(|| {
                RewardCandidateError::InvalidInput(
                    "approved amount is required for amount approval".to_string(),
                )
            })?;
            if amount < BigDecimal::from(0) {
                return Err(RewardCandidateError::InvalidInput(
                    "approved amount cannot be negative".to_string(),
                ));
            }
            Some(amount)
        }
        REWARD_STATUS_AMOUNT_REJECTED => None,
        _ => unreachable!("amount status normalization returned unsupported status"),
    };

    conn.transaction::<_, RewardCandidateError, _>(|conn| {
        Box::pin(async move {
            let existing = reward_candidate_repository::find_candidate(conn, candidate_id).await?;
            if existing.status == target_status {
                return Ok(existing);
            }

            if existing.status != REWARD_STATUS_TEACHER_APPROVED {
                return Err(RewardCandidateError::InvalidStatus(
                    "reward amount can be decided only after teacher approval".to_string(),
                ));
            }

            let teacher_user_ids = candidate_teacher_user_ids(&existing);
            ensure_no_active_reward_fraud_block(
                conn,
                teacher_user_ids.as_slice(),
                existing.course_id,
                &existing.event_type,
                existing.source_organization_id,
            )
            .await?;

            let updated = reward_candidate_repository::update_amount_decision(
                conn,
                candidate_id,
                actor_user_id,
                &target_status,
                approved_amount,
                request.decision_reason.as_deref(),
                Utc::now(),
            )
            .await
            .map_err(RewardCandidateError::from)?;

            if target_status == REWARD_STATUS_AMOUNT_APPROVED {
                reward_execution_job_repository::enqueue_reward_execution_job(conn, candidate_id)
                    .await?;
            }

            Ok(updated)
        })
    })
    .await
}

pub async fn list_course_reward_candidates(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    request: ListRewardCandidatesRequest,
) -> Result<Vec<RewardCandidate>, RewardCandidateError> {
    ensure_course_exists(conn, course_id).await?;

    let can_manage = user_permission_course_request(
        conn,
        actor_user_id,
        course_id,
        &Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
    )
    .await?
        || user_permission_course_request(
            conn,
            actor_user_id,
            course_id,
            &Permissions::MANAGE_COURSE_REWARD_RULES.to_string(),
        )
        .await?;

    if !can_manage {
        ensure_exact_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::VIEW_COURSE_REWARD_STATUS,
        )
        .await?;
    }

    let status = match request.status {
        Some(status) => Some(normalize_reward_status(&status)?),
        None => None,
    };
    let student_user_id = if can_manage {
        request.student_user_id
    } else {
        Some(actor_user_id)
    };

    reward_candidate_repository::list_candidates(
        conn,
        RewardCandidateFilter {
            course_id: Some(course_id),
            student_user_id,
            status,
            limit: request.limit,
            offset: request.offset,
        },
    )
    .await
    .map_err(RewardCandidateError::from)
}

async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    source_organization_id: Option<i32>,
    source_scope: &str,
    request: SubmitRewardCandidateRequest,
) -> Result<RewardCandidate, RewardCandidateError> {
    let event_type = normalize_reward_event_type(&request.event_type)?;
    let idempotency_key = normalize_idempotency_key(
        request.idempotency_key,
        course_id,
        request.student_user_id,
        &event_type,
    )?;
    let evidence = request.evidence.unwrap_or_else(|| json!({}));

    if let Some(existing) =
        reward_candidate_repository::find_candidate_by_idempotency_key(conn, &idempotency_key)
            .await?
    {
        if existing.course_id == course_id
            && existing.student_user_id == request.student_user_id
            && existing.event_type == event_type
        {
            return Ok(existing);
        }

        return Err(RewardCandidateError::InvalidInput(
            "idempotency key is already used by another reward candidate".to_string(),
        ));
    }

    ensure_no_active_reward_fraud_block(
        conn,
        &[actor_user_id],
        course_id,
        &event_type,
        source_organization_id,
    )
    .await?;
    ensure_reward_target_eligible(conn, request.student_user_id, course_id, &event_type).await?;
    ensure_reward_evidence_is_eligible(&event_type, &evidence)?;
    ensure_no_prior_active_reward_candidate(conn, request.student_user_id, course_id, &event_type)
        .await?;

    let new_candidate = NewRewardCandidate {
        course_id,
        student_user_id: request.student_user_id,
        submitter_user_id: actor_user_id,
        source_scope: source_scope.to_string(),
        source_organization_id,
        event_type,
        idempotency_key,
        evidence,
        status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
    };

    reward_candidate_repository::create_candidate(conn, new_candidate)
        .await
        .map_err(RewardCandidateError::from)
}

fn candidate_teacher_user_ids(candidate: &RewardCandidate) -> Vec<i32> {
    let mut user_ids = vec![candidate.submitter_user_id];
    if let Some(teacher_approver_user_id) = candidate.teacher_approver_user_id {
        if !user_ids.contains(&teacher_approver_user_id) {
            user_ids.push(teacher_approver_user_id);
        }
    }
    user_ids
}

async fn ensure_no_active_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    teacher_user_ids: &[i32],
    course_id: i32,
    event_type: &str,
    source_organization_id: Option<i32>,
) -> Result<(), RewardCandidateError> {
    if !teacher_user_ids.is_empty()
        && has_active_teacher_reward_fraud_block(conn, teacher_user_ids).await?
    {
        return Err(RewardCandidateError::InvalidStatus(
            "teacher reward activity is blocked by platform fraud controls".to_string(),
        ));
    }

    if has_active_course_reward_fraud_block(conn, course_id).await? {
        return Err(RewardCandidateError::InvalidStatus(
            "course reward activity is blocked by platform fraud controls".to_string(),
        ));
    }

    let organization_ids =
        course_organization_ids_with_source(conn, course_id, source_organization_id).await?;
    if !organization_ids.is_empty()
        && has_active_organization_reward_fraud_block(conn, organization_ids.as_slice()).await?
    {
        return Err(RewardCandidateError::InvalidStatus(
            "organization reward activity is blocked by platform fraud controls".to_string(),
        ));
    }

    let active_policy_ids = active_reward_policy_ids_for_course_event(conn, course_id, event_type)
        .await
        .map_err(RewardCandidateError::from)?;
    if !active_policy_ids.is_empty()
        && has_active_reward_policy_fraud_block(conn, active_policy_ids.as_slice()).await?
    {
        return Err(RewardCandidateError::InvalidStatus(
            "reward policy activity is blocked by platform fraud controls".to_string(),
        ));
    }

    Ok(())
}

async fn ensure_course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<(), RewardCandidateError> {
    courses::table
        .find(course_id)
        .select(courses::id)
        .first::<i32>(conn)
        .await?;
    Ok(())
}

async fn ensure_course_attached_to_organization(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) -> Result<(), RewardCandidateError> {
    let attached = diesel::select(exists(
        courses_organizations::table
            .filter(courses_organizations::course_id.eq(course_id))
            .filter(courses_organizations::organization_id.eq(organization_id)),
    ))
    .get_result(conn)
    .await?;

    if attached {
        Ok(())
    } else {
        Err(RewardCandidateError::InvalidInput(
            "course is not attached to the organization".to_string(),
        ))
    }
}

async fn course_organization_ids_with_source(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    source_organization_id: Option<i32>,
) -> Result<Vec<i32>, RewardCandidateError> {
    let mut organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;

    if let Some(source_organization_id) = source_organization_id {
        if !organization_ids.contains(&source_organization_id) {
            organization_ids.push(source_organization_id);
        }
    }

    Ok(organization_ids)
}

async fn has_active_teacher_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    teacher_user_ids: &[i32],
) -> Result<bool, RewardCandidateError> {
    diesel::select(exists(
        reward_fraud_blocks::table
            .filter(reward_fraud_blocks::scope_type.eq(REWARD_FRAUD_BLOCK_SCOPE_TEACHER))
            .filter(reward_fraud_blocks::revoked_at.is_null())
            .filter(
                reward_fraud_blocks::expires_at
                    .is_null()
                    .or(reward_fraud_blocks::expires_at.gt(Utc::now())),
            )
            .filter(reward_fraud_blocks::teacher_user_id.eq_any(teacher_user_ids)),
    ))
    .get_result(conn)
    .await
    .map_err(RewardCandidateError::from)
}

async fn has_active_course_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<bool, RewardCandidateError> {
    diesel::select(exists(
        reward_fraud_blocks::table
            .filter(reward_fraud_blocks::scope_type.eq(REWARD_FRAUD_BLOCK_SCOPE_COURSE))
            .filter(reward_fraud_blocks::revoked_at.is_null())
            .filter(
                reward_fraud_blocks::expires_at
                    .is_null()
                    .or(reward_fraud_blocks::expires_at.gt(Utc::now())),
            )
            .filter(reward_fraud_blocks::course_id.eq(course_id)),
    ))
    .get_result(conn)
    .await
    .map_err(RewardCandidateError::from)
}

async fn has_active_organization_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    organization_ids: &[i32],
) -> Result<bool, RewardCandidateError> {
    diesel::select(exists(
        reward_fraud_blocks::table
            .filter(reward_fraud_blocks::scope_type.eq(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION))
            .filter(reward_fraud_blocks::revoked_at.is_null())
            .filter(
                reward_fraud_blocks::expires_at
                    .is_null()
                    .or(reward_fraud_blocks::expires_at.gt(Utc::now())),
            )
            .filter(reward_fraud_blocks::organization_id.eq_any(organization_ids)),
    ))
    .get_result(conn)
    .await
    .map_err(RewardCandidateError::from)
}

async fn has_active_reward_policy_fraud_block(
    conn: &mut AsyncPgConnection,
    reward_policy_ids: &[i64],
) -> Result<bool, RewardCandidateError> {
    diesel::select(exists(
        reward_fraud_blocks::table
            .filter(reward_fraud_blocks::scope_type.eq(REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY))
            .filter(reward_fraud_blocks::revoked_at.is_null())
            .filter(
                reward_fraud_blocks::expires_at
                    .is_null()
                    .or(reward_fraud_blocks::expires_at.gt(Utc::now())),
            )
            .filter(reward_fraud_blocks::reward_policy_id.eq_any(reward_policy_ids)),
    ))
    .get_result(conn)
    .await
    .map_err(RewardCandidateError::from)
}

async fn ensure_reward_target_eligible(
    conn: &mut AsyncPgConnection,
    student_user_id: i32,
    course_id: i32,
    event_type: &str,
) -> Result<(), RewardCandidateError> {
    let (_user_id, email_verified) = users::table
        .find(student_user_id)
        .select((users::id, users::email_verified))
        .first::<(i32, bool)>(conn)
        .await?;

    if !email_verified {
        return Err(RewardCandidateError::InvalidInput(
            "reward recipient must have a verified email".to_string(),
        ));
    }

    let can_view_reward_status = user_permission_course_request(
        conn,
        student_user_id,
        course_id,
        &Permissions::VIEW_COURSE_REWARD_STATUS.to_string(),
    )
    .await?;

    if can_view_reward_status {
        Ok(())
    } else {
        Err(RewardCandidateError::InvalidInput(
            "reward recipient is not eligible for this course".to_string(),
        ))
    }?;

    ensure_active_reward_policy(conn, course_id, event_type).await
}

async fn active_reward_policy_ids_for_course_event(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> QueryResult<Vec<i64>> {
    let mut policy_ids = reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .select(reward_policies::id)
        .load::<i64>(conn)
        .await?;

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;
    if !organization_ids.is_empty() {
        policy_ids.extend(
            reward_policies::table
                .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
                .filter(reward_policies::organization_id.eq_any(organization_ids))
                .filter(reward_policies::course_id.is_null())
                .filter(reward_policies::event_type.eq(event_type))
                .filter(reward_policies::active.eq(true))
                .select(reward_policies::id)
                .load::<i64>(conn)
                .await?,
        );
    }

    policy_ids.extend(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
            .filter(reward_policies::organization_id.is_null())
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true))
            .select(reward_policies::id)
            .load::<i64>(conn)
            .await?,
    );

    Ok(policy_ids)
}

async fn ensure_active_reward_policy(
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

async fn ensure_no_prior_active_reward_candidate(
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

fn ensure_reward_evidence_is_eligible(
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

fn ensure_evidence_number_at_least(
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

async fn ensure_course_submission_permission(
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

async fn ensure_course_teacher_approval_permission(
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

async fn ensure_exact_course_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: Permissions,
) -> Result<(), RewardCandidateError> {
    let permission_name = permission.to_string();
    if user_permission_course_request(conn, user_id, course_id, &permission_name).await? {
        Ok(())
    } else {
        Err(RewardCandidateError::PermissionDenied(permission_name))
    }
}

async fn ensure_organization_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<(), RewardCandidateError> {
    let permission_name = permission.to_string();
    if user_permission_organization_request(conn, user_id, organization_id, &permission_name)
        .await?
    {
        Ok(())
    } else {
        Err(RewardCandidateError::PermissionDenied(permission_name))
    }
}

async fn ensure_platform_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: Permissions,
) -> Result<(), RewardCandidateError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, user_id, &permission_name).await? {
        Ok(())
    } else {
        Err(RewardCandidateError::PermissionDenied(permission_name))
    }
}

fn normalize_reward_event_type(event_type: &str) -> Result<String, RewardCandidateError> {
    let normalized = event_type
        .trim()
        .to_ascii_lowercase()
        .replace('-', "_")
        .replace(' ', "_");
    match normalized.as_str() {
        REWARD_EVENT_ASSESSMENT_COMPLETION
        | REWARD_EVENT_COURSE_COMPLETION
        | REWARD_EVENT_MANUAL_COMPLETION
        | REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT => Ok(normalized),
        _ => Err(RewardCandidateError::InvalidInput(
            "unsupported reward event type".to_string(),
        )),
    }
}

fn normalize_teacher_decision_status(status: &str) -> Result<String, RewardCandidateError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "approved" | REWARD_STATUS_TEACHER_APPROVED => {
            Ok(REWARD_STATUS_TEACHER_APPROVED.to_string())
        }
        "rejected" | REWARD_STATUS_TEACHER_REJECTED => {
            Ok(REWARD_STATUS_TEACHER_REJECTED.to_string())
        }
        _ => Err(RewardCandidateError::InvalidStatus(
            "unsupported teacher reward decision status".to_string(),
        )),
    }
}

fn normalize_amount_decision_status(status: &str) -> Result<String, RewardCandidateError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "approved" | REWARD_STATUS_AMOUNT_APPROVED => Ok(REWARD_STATUS_AMOUNT_APPROVED.to_string()),
        "rejected" | REWARD_STATUS_AMOUNT_REJECTED => Ok(REWARD_STATUS_AMOUNT_REJECTED.to_string()),
        _ => Err(RewardCandidateError::InvalidStatus(
            "unsupported reward amount decision status".to_string(),
        )),
    }
}

fn normalize_reward_status(status: &str) -> Result<String, RewardCandidateError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        REWARD_STATUS_PENDING_TEACHER_APPROVAL
        | REWARD_STATUS_TEACHER_APPROVED
        | REWARD_STATUS_TEACHER_REJECTED
        | REWARD_STATUS_AMOUNT_APPROVED
        | REWARD_STATUS_AMOUNT_REJECTED
        | REWARD_STATUS_ADJUSTED
        | REWARD_STATUS_TOKEN_PENDING
        | REWARD_STATUS_TOKEN_CONFIRMED
        | REWARD_STATUS_WALLET_CREDITED
        | REWARD_STATUS_NOTIFIED
        | REWARD_STATUS_COMPLETED
        | REWARD_STATUS_NEEDS_RECONCILIATION
        | REWARD_STATUS_FAILED => Ok(normalized),
        _ => Err(RewardCandidateError::InvalidStatus(
            "unsupported reward candidate status".to_string(),
        )),
    }
}

fn normalize_idempotency_key(
    idempotency_key: Option<String>,
    course_id: i32,
    student_user_id: i32,
    event_type: &str,
) -> Result<String, RewardCandidateError> {
    match idempotency_key {
        Some(key) => {
            let trimmed = key.trim();
            if trimmed.is_empty() {
                Err(RewardCandidateError::InvalidInput(
                    "idempotency key cannot be blank".to_string(),
                ))
            } else {
                Ok(trimmed.to_string())
            }
        }
        None => Ok(format!(
            "{}:{}:{}:manual",
            event_type, course_id, student_user_id
        )),
    }
}
