#[actix_web::test]
async fn test_execution_job_enqueue_is_idempotent() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "exec_idem_student").await;
    let course = create_course(&mut conn, &unique_string("exec_idem_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("exec_idem_cand")),
    )
    .await
    .unwrap();

    let job1 =
        reward_execution_job_repository::enqueue_reward_execution_job(&mut conn, candidate.id)
            .await
            .unwrap();
    let job2 =
        reward_execution_job_repository::enqueue_reward_execution_job(&mut conn, candidate.id)
            .await
            .unwrap();

    assert_eq!(job1.id, job2.id);
}

// ── reward_audit_event_repository ──

#[actix_web::test]
async fn test_audit_event_create_and_list() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "audit_student").await;
    let course = create_course(&mut conn, &unique_string("audit_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("audit_cand")),
    )
    .await
    .unwrap();

    let event1 = reward_audit_event_repository::create_reward_audit_event(
        &mut conn,
        NewRewardAuditEvent {
            reward_candidate_id: candidate.id,
            actor_user_id: Some(student.id()),
            event_type: REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED.to_string(),
            from_status: None,
            to_status: candidate.status.clone(),
            reason: None,
            metadata: json!({}),
        },
    )
    .await
    .unwrap();

    let event2 = reward_audit_event_repository::create_reward_audit_event(
        &mut conn,
        NewRewardAuditEvent {
            reward_candidate_id: candidate.id,
            actor_user_id: Some(student.id()),
            event_type: "teacher_decision".to_string(),
            from_status: Some(candidate.status),
            to_status: "teacher_approved".to_string(),
            reason: Some("looks good".to_string()),
            metadata: json!({"source": "test"}),
        },
    )
    .await
    .unwrap();

    let events = reward_audit_event_repository::list_reward_audit_events(&mut conn, candidate.id)
        .await
        .unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].id, event1.id);
    assert_eq!(events[1].id, event2.id);
}
