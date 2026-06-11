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
