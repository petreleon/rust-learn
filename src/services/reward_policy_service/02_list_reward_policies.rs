pub async fn list_reward_policies(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: ListRewardPoliciesRequest,
) -> Result<Vec<RewardPolicy>, RewardPolicyError> {
    ensure_platform_permission(conn, actor_user_id, Permissions::SET_REWARD_POLICY).await?;

    let scope_type = match request.scope_type {
        Some(scope_type) => Some(normalize_scope_type(&scope_type)?),
        None => None,
    };
    let event_type = match request.event_type {
        Some(event_type) => Some(normalize_event_type(&event_type)?),
        None => None,
    };

    reward_policy_repository::list_policies(
        conn,
        RewardPolicyFilter {
            scope_type,
            organization_id: request.organization_id,
            course_id: request.course_id,
            event_type,
            active: request.active,
            limit: request.limit,
            offset: request.offset,
        },
    )
    .await
    .map_err(RewardPolicyError::from)
}

async fn ensure_platform_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: Permissions,
) -> Result<(), RewardPolicyError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, user_id, &permission_name).await? {
        Ok(())
    } else {
        Err(RewardPolicyError::PermissionDenied(permission_name))
    }
}

async fn validate_scope_references(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(), RewardPolicyError> {
    match scope_type {
        REWARD_POLICY_SCOPE_PLATFORM => {
            if organization_id.is_some() || course_id.is_some() {
                return Err(RewardPolicyError::InvalidInput(
                    "platform reward policy cannot include organization or course scope"
                        .to_string(),
                ));
            }
        }
        REWARD_POLICY_SCOPE_ORGANIZATION => {
            let organization_id = organization_id.ok_or_else(|| {
                RewardPolicyError::InvalidInput(
                    "organization reward policy requires organization_id".to_string(),
                )
            })?;
            if course_id.is_some() {
                return Err(RewardPolicyError::InvalidInput(
                    "organization reward policy cannot include course_id".to_string(),
                ));
            }
            organizations::table
                .find(organization_id)
                .select(organizations::id)
                .first::<i32>(conn)
                .await?;
        }
        REWARD_POLICY_SCOPE_COURSE => {
            let course_id = course_id.ok_or_else(|| {
                RewardPolicyError::InvalidInput(
                    "course reward policy requires course_id".to_string(),
                )
            })?;
            courses::table
                .find(course_id)
                .select(courses::id)
                .first::<i32>(conn)
                .await?;
            if let Some(organization_id) = organization_id {
                organizations::table
                    .find(organization_id)
                    .select(organizations::id)
                    .first::<i32>(conn)
                    .await?;
            }
        }
        _ => unreachable!("scope type was normalized before validation"),
    }

    Ok(())
}

fn validate_amounts(
    token_amount: &BigDecimal,
    multiplier: &BigDecimal,
    max_payout: Option<&BigDecimal>,
    cooldown_seconds: i64,
) -> Result<(), RewardPolicyError> {
    if token_amount < &BigDecimal::from(0) {
        return Err(RewardPolicyError::InvalidInput(
            "token amount cannot be negative".to_string(),
        ));
    }
    if multiplier <= &BigDecimal::from(0) {
        return Err(RewardPolicyError::InvalidInput(
            "multiplier must be greater than zero".to_string(),
        ));
    }
    if let Some(max_payout) = max_payout {
        if max_payout < &BigDecimal::from(0) {
            return Err(RewardPolicyError::InvalidInput(
                "max payout cannot be negative".to_string(),
            ));
        }
    }
    if cooldown_seconds < 0 {
        return Err(RewardPolicyError::InvalidInput(
            "cooldown seconds cannot be negative".to_string(),
        ));
    }

    Ok(())
}

fn normalize_scope_type(scope_type: &str) -> Result<String, RewardPolicyError> {
    let normalized = scope_type.trim().to_ascii_lowercase();
    match normalized.as_str() {
        REWARD_POLICY_SCOPE_PLATFORM
        | REWARD_POLICY_SCOPE_ORGANIZATION
        | REWARD_POLICY_SCOPE_COURSE => Ok(normalized),
        _ => Err(RewardPolicyError::InvalidInput(
            "unsupported reward policy scope type".to_string(),
        )),
    }
}
