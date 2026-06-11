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
        .replace(['-', ' '], "_");
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
