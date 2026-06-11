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
