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
