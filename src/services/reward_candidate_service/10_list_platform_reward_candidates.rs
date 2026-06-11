pub async fn list_platform_reward_candidates(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: PlatformRewardCandidatesRequest,
) -> Result<PlatformRewardCandidatesResponse, RewardCandidateError> {
    ensure_platform_permission(conn, actor_user_id, Permissions::VIEW_REWARD_AUDIT).await?;

    let status = match request.status {
        Some(ref s) => Some(normalize_reward_status(s)?),
        None => None,
    };
    let search = request
        .search
        .as_ref()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty());
    let limit = request.limit.unwrap_or(25).clamp(1, 100);
    let offset = request.offset.unwrap_or(0).max(0);

    let can_approve_amount = user_permission_platform_request(
        conn,
        actor_user_id,
        &Permissions::APPROVE_REWARD_AMOUNT.to_string(),
    )
    .await?;

    let candidates = reward_candidate_repository::list_candidates(
        conn,
        RewardCandidateFilter {
            course_id: None,
            student_user_id: None,
            status: status.clone(),
            limit: None,
            offset: None,
        },
    )
    .await
    .map_err(RewardCandidateError::from)?;

    let _total_all = reward_candidate_repository::count_candidates(
        conn,
        RewardCandidateFilter {
            course_id: None,
            student_user_id: None,
            status: status.clone(),
            limit: None,
            offset: None,
        },
    )
    .await
    .map_err(RewardCandidateError::from)?;

    let mut items = build_platform_reward_candidate_items(conn, &candidates).await?;

    if let Some(ref s) = search {
        items.retain(|item| {
            item.student.name.to_lowercase().contains(s)
                || item.student.email.to_lowercase().contains(s)
                || item.course.title.to_lowercase().contains(s)
                || format!("{}", item.id).contains(s)
        });
    }

    let total = items.len() as i64;
    let paged = items
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<_>>();

    Ok(PlatformRewardCandidatesResponse {
        candidates: paged,
        total,
        limit,
        offset,
        status,
        search,
        operator_permissions: PlatformRewardCandidatePermissions {
            can_view_candidates: true,
            can_approve_amount,
        },
    })
}
