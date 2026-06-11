async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    source_organization_id: Option<i32>,
    source_scope: &str,
    request: SubmitRewardCandidateRequest,
) -> Result<RewardCandidate, RewardCandidateError> {
    let event_type = normalize_reward_event_type(&request.event_type)?;
    let idempotency_key = normalize_idempotency_key(
        request.idempotency_key,
        course_id,
        request.student_user_id,
        &event_type,
    )?;
    let evidence = request.evidence.unwrap_or_else(|| json!({}));

    if let Some(existing) =
        reward_candidate_repository::find_candidate_by_idempotency_key(conn, &idempotency_key)
            .await?
    {
        if existing.course_id == course_id
            && existing.student_user_id == request.student_user_id
            && existing.event_type == event_type
        {
            log::info!(
                "event=reward_candidate_idempotent_replay candidate_id={} actor_user_id={} student_user_id={} course_id={} status={} event_type={} source_scope={} source_organization_id={:?} idempotency_key={}",
                existing.id,
                actor_user_id,
                existing.student_user_id,
                existing.course_id,
                existing.status,
                existing.event_type,
                existing.source_scope,
                existing.source_organization_id,
                existing.idempotency_key
            );
            return Ok(existing);
        }

        return Err(RewardCandidateError::InvalidInput(
            "idempotency key is already used by another reward candidate".to_string(),
        ));
    }

    ensure_no_active_reward_fraud_block(
        conn,
        &[actor_user_id],
        course_id,
        &event_type,
        source_organization_id,
    )
    .await?;
    ensure_reward_target_eligible(conn, request.student_user_id, course_id, &event_type).await?;
    ensure_reward_evidence_is_eligible(&event_type, &evidence)?;
    ensure_no_prior_active_reward_candidate(conn, request.student_user_id, course_id, &event_type)
        .await?;

    let new_candidate = NewRewardCandidate {
        course_id,
        student_user_id: request.student_user_id,
        submitter_user_id: actor_user_id,
        source_scope: source_scope.to_string(),
        source_organization_id,
        event_type,
        idempotency_key,
        evidence,
        status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
    };

    let created = conn
        .transaction::<_, RewardCandidateError, _>(|conn| {
            Box::pin(async move {
                let created = reward_candidate_repository::create_candidate(conn, new_candidate)
                    .await
                    .map_err(RewardCandidateError::from)?;
                reward_audit_event_repository::create_reward_audit_event(
                    conn,
                    NewRewardAuditEvent {
                        reward_candidate_id: created.id,
                        actor_user_id: Some(actor_user_id),
                        event_type: REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED.to_string(),
                        from_status: None,
                        to_status: created.status.clone(),
                        reason: None,
                        metadata: json!({
                            "course_id": created.course_id,
                            "student_user_id": created.student_user_id,
                            "source_scope": created.source_scope.clone(),
                            "source_organization_id": created.source_organization_id,
                            "event_type": created.event_type.clone(),
                            "idempotency_key": created.idempotency_key.clone(),
                        }),
                    },
                )
                .await
                .map_err(RewardCandidateError::from)?;

                Ok(created)
            })
        })
        .await?;

    log::info!(
        "event=reward_candidate_submitted candidate_id={} actor_user_id={} student_user_id={} course_id={} status={} event_type={} source_scope={} source_organization_id={:?} idempotency_key={}",
        created.id,
        actor_user_id,
        created.student_user_id,
        created.course_id,
        created.status,
        created.event_type,
        created.source_scope,
        created.source_organization_id,
        created.idempotency_key
    );

    Ok(created)
}

fn candidate_teacher_user_ids(candidate: &RewardCandidate) -> Vec<i32> {
    let mut user_ids = vec![candidate.submitter_user_id];
    if let Some(teacher_approver_user_id) = candidate.teacher_approver_user_id {
        if !user_ids.contains(&teacher_approver_user_id) {
            user_ids.push(teacher_approver_user_id);
        }
    }
    user_ids
}
