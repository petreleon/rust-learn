async fn select_payout_method(
    conn: &mut AsyncPgConnection,
    policy: &RewardPolicy,
) -> Result<String, RewardExecutionError> {
    match policy.payment_strategy.as_str() {
        REWARD_PAYMENT_TREASURY_TRANSFER => {
            if has_presigner_contract(conn).await? {
                Ok(REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER.to_string())
            } else {
                Ok(REWARD_PAYOUT_METHOD_TREASURY_TRANSFER.to_string())
            }
        }
        REWARD_PAYMENT_MINT => Ok(REWARD_PAYOUT_METHOD_MINT.to_string()),
        REWARD_PAYMENT_OFF_CHAIN => Ok(REWARD_PAYOUT_METHOD_OFF_CHAIN.to_string()),
        _ => Err(RewardExecutionError::InvalidInput(
            "unsupported reward payment strategy".to_string(),
        )),
    }
}

async fn has_presigner_contract(
    conn: &mut AsyncPgConnection,
) -> Result<bool, RewardExecutionError> {
    let address = get_persistent_state(conn, "learn_token_presigner_address").await?;
    Ok(address
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false))
}

async fn resolve_active_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> QueryResult<Option<RewardPolicy>> {
    if let Some(policy) = reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()?
    {
        return Ok(Some(policy));
    }

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;
    if !organization_ids.is_empty() {
        if let Some(policy) = reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
            .filter(reward_policies::organization_id.eq_any(organization_ids))
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true))
            .order((
                reward_policies::version.desc(),
                reward_policies::created_at.desc(),
            ))
            .first::<RewardPolicy>(conn)
            .await
            .optional()?
        {
            return Ok(Some(policy));
        }
    }

    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
        .filter(reward_policies::organization_id.is_null())
        .filter(reward_policies::course_id.is_null())
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()
}

#[cfg(test)]
mod tests;
