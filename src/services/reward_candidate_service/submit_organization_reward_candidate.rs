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

    let updated = conn
        .transaction::<_, RewardCandidateError, _>(|conn| {
            Box::pin(async move {
                let existing =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
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
                let from_status = existing.status.clone();

                ensure_no_active_reward_fraud_block(
                    conn,
                    &[actor_user_id],
                    existing.course_id,
                    &existing.event_type,
                    existing.source_organization_id,
                )
                .await?;

                let updated = reward_candidate_repository::update_teacher_decision(
                    conn,
                    candidate_id,
                    actor_user_id,
                    &target_status,
                    request.decision_reason.as_deref(),
                    Utc::now(),
                )
                .await
                .map_err(RewardCandidateError::from)?;

                reward_audit_event_repository::create_reward_audit_event(
                    conn,
                    NewRewardAuditEvent {
                        reward_candidate_id: updated.id,
                        actor_user_id: Some(actor_user_id),
                        event_type: REWARD_AUDIT_EVENT_TEACHER_DECISION.to_string(),
                        from_status: Some(from_status),
                        to_status: updated.status.clone(),
                        reason: request.decision_reason.clone(),
                        metadata: json!({}),
                    },
                )
                .await
                .map_err(RewardCandidateError::from)?;

                Ok(updated)
            })
        })
        .await?;

    log::info!(
        "event=reward_candidate_teacher_decision candidate_id={} actor_user_id={} student_user_id={} course_id={} status={} event_type={} source_scope={} source_organization_id={:?}",
        updated.id,
        actor_user_id,
        updated.student_user_id,
        updated.course_id,
        updated.status,
        updated.event_type,
        updated.source_scope,
        updated.source_organization_id
    );

    Ok(updated)
}
