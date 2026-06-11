fn normalize_block_request(
    request: RewardFraudBlockRequest,
) -> Result<NormalizedRewardFraudBlock, RewardFraudBlockError> {
    let scope_type = normalize_scope_type(&request.scope_type)?;
    let reason = request.reason.trim().to_string();
    if reason.is_empty() {
        return Err(RewardFraudBlockError::InvalidInput(
            "block reason is required".to_string(),
        ));
    }

    let evidence_reference = request
        .evidence_reference
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let target_count = [
        request.teacher_user_id.is_some(),
        request.organization_id.is_some(),
        request.course_id.is_some(),
        request.reward_policy_id.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    if target_count != 1 {
        return Err(RewardFraudBlockError::InvalidInput(
            "exactly one fraud block target is required".to_string(),
        ));
    }

    match scope_type.as_str() {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER if request.teacher_user_id.is_some() => {}
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION if request.organization_id.is_some() => {}
        REWARD_FRAUD_BLOCK_SCOPE_COURSE if request.course_id.is_some() => {}
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY if request.reward_policy_id.is_some() => {}
        _ => {
            return Err(RewardFraudBlockError::InvalidInput(
                "fraud block target does not match scope type".to_string(),
            ));
        }
    }

    Ok(NormalizedRewardFraudBlock {
        scope_type,
        teacher_user_id: request.teacher_user_id,
        organization_id: request.organization_id,
        course_id: request.course_id,
        reward_policy_id: request.reward_policy_id,
        reason,
        evidence_reference,
        expires_at: request.expires_at,
    })
}

fn normalize_scope_type(scope_type: &str) -> Result<String, RewardFraudBlockError> {
    match scope_type.trim() {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER => Ok(REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string()),
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => {
            Ok(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION.to_string())
        }
        REWARD_FRAUD_BLOCK_SCOPE_COURSE => Ok(REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string()),
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => {
            Ok(REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY.to_string())
        }
        _ => Err(RewardFraudBlockError::InvalidInput(
            "unsupported fraud block scope type".to_string(),
        )),
    }
}

async fn ensure_scope_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    scope_type: &str,
) -> Result<(), RewardFraudBlockError> {
    let permissions = required_permissions_for_scope(scope_type)?;
    for permission in permissions.iter() {
        if user_permission_platform_request(conn, actor_user_id, &permission.to_string()).await? {
            return Ok(());
        }
    }

    Err(RewardFraudBlockError::PermissionDenied(
        permissions
            .into_iter()
            .map(|permission| permission.to_string())
            .collect::<Vec<_>>()
            .join(" or "),
    ))
}

async fn ensure_reward_fraud_report_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<(), RewardFraudBlockError> {
    for permission in [
        Permissions::VIEW_REWARD_AUDIT,
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
    ] {
        if user_permission_platform_request(conn, actor_user_id, &permission.to_string()).await? {
            return Ok(());
        }
    }

    Err(RewardFraudBlockError::PermissionDenied(format!(
        "{} or {}",
        Permissions::VIEW_REWARD_AUDIT,
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS
    )))
}

fn required_permissions_for_scope(
    scope_type: &str,
) -> Result<Vec<Permissions>, RewardFraudBlockError> {
    match scope_type {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER => Ok(vec![
            Permissions::BLOCK_REWARD_TEACHER,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
        ]),
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => Ok(vec![
            Permissions::BLOCK_REWARD_ORGANIZATION,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
        ]),
        REWARD_FRAUD_BLOCK_SCOPE_COURSE | REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => {
            Ok(vec![Permissions::MANAGE_REWARD_FRAUD_BLOCKS])
        }
        _ => Err(RewardFraudBlockError::InvalidInput(
            "unsupported fraud block scope type".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests;
