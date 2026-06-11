async fn load_learner_course_organizations(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseCatalogOrganization>, LearnerCourseCatalogError> {
    let rows = courses_organizations::table
        .inner_join(
            organizations::table.on(courses_organizations::organization_id.eq(organizations::id)),
        )
        .filter(courses_organizations::course_id.eq(course_id))
        .order(courses_organizations::order.asc())
        .select((organizations::id, organizations::name))
        .load::<(i32, String)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| LearnerCourseCatalogOrganization { id, name })
        .collect())
}

async fn load_learner_course_teachers(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseCatalogTeacher>, LearnerCourseCatalogError> {
    let rows = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .inner_join(users::table.on(user_role_course::user_id.eq(users::id.nullable())))
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("TEACHER"))
        .order(users::name.asc())
        .select((users::id, users::name))
        .load::<(i32, String)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| LearnerCourseCatalogTeacher { id, name })
        .collect())
}

async fn load_learner_course_content_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<LearnerCourseContentSummary, LearnerCourseCatalogError> {
    let chapter_count = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let content_types = contents::table
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(chapters::course_id.eq(course_id))
        .order(contents::content_type.asc())
        .select(contents::content_type)
        .load::<String>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut unique_types = BTreeSet::new();
    for content_type in &content_types {
        unique_types.insert(content_type.clone());
    }

    Ok(LearnerCourseContentSummary {
        chapter_count: chapter_count as usize,
        content_count: content_types.len(),
        content_types: unique_types.into_iter().collect(),
        has_content: !content_types.is_empty(),
    })
}

async fn load_learner_course_reward_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<LearnerCourseRewardSummary, LearnerCourseCatalogError> {
    let rows = reward_policies::table
        .filter(reward_policies::course_id.eq(course_id))
        .filter(reward_policies::active.eq(true))
        .order(reward_policies::event_type.asc())
        .select((
            reward_policies::event_type,
            reward_policies::token_amount,
            reward_policies::payment_strategy,
        ))
        .load::<(String, BigDecimal, String)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut event_types = BTreeSet::new();
    let mut token_amounts = BTreeSet::new();
    let mut payment_strategies = BTreeSet::new();

    for (event_type, token_amount, payment_strategy) in &rows {
        event_types.insert(event_type.clone());
        token_amounts.insert(token_amount.to_string());
        payment_strategies.insert(payment_strategy.clone());
    }

    Ok(LearnerCourseRewardSummary {
        available: !rows.is_empty(),
        active_policy_count: rows.len(),
        event_types: event_types.into_iter().collect(),
        token_amounts: token_amounts.into_iter().collect(),
        payment_strategies: payment_strategies.into_iter().collect(),
    })
}

async fn build_learner_course_access(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<LearnerCourseAccessSummary, LearnerCourseCatalogError> {
    let can_view_course = user_has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_COURSE,
    )
    .await?;
    let can_view_content = user_has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_CONTENT,
    )
    .await?;
    let can_view_rewards = user_has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_COURSE_REWARD_STATUS,
    )
    .await?;
    let can_request_join = user_has_any_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &[Permissions::REQUEST_JOIN_COURSE, Permissions::JOIN_COURSE],
    )
    .await?;

    Ok(LearnerCourseAccessSummary {
        can_view_course,
        can_view_content,
        can_view_rewards,
        can_request_join,
    })
}
