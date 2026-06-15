use crate::support::*;

// ── reward_candidate_repository ──

#[actix_web::test]
async fn test_candidate_create_and_find() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "candidate_student").await;
    let course = create_course(&mut conn, &unique_string("candidate_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        NewRewardCandidate {
            course_id: course.id,
            student_user_id: student.id(),
            submitter_user_id: student.id(),
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("candidate_key"),
            evidence: json!({"completion_percentage": 100.0}),
            status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
        },
    )
    .await
    .expect("failed to create candidate");

    assert_eq!(candidate.course_id, course.id);
    assert_eq!(candidate.student_user_id, student.id());
    assert_eq!(candidate.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);

    let found = reward_candidate_repository::find_candidate(&mut conn, candidate.id)
        .await
        .expect("candidate not found by id");
    assert_eq!(found.id, candidate.id);
}

#[actix_web::test]
async fn test_candidate_find_by_idempotency_key() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_idem").await;
    let course = create_course(&mut conn, &unique_string("cand_idem_course")).await;
    let key = unique_string("idem_key");

    let created = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &key),
    )
    .await
    .unwrap();

    let found = reward_candidate_repository::find_candidate_by_idempotency_key(&mut conn, &key)
        .await
        .unwrap()
        .expect("candidate not found by idempotency key");
    assert_eq!(found.id, created.id);

    let none =
        reward_candidate_repository::find_candidate_by_idempotency_key(&mut conn, "nonexistent")
            .await
            .unwrap();
    assert!(none.is_none());
}

#[actix_web::test]
async fn test_candidate_list_and_count() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_list").await;
    let course_a = create_course(&mut conn, &unique_string("cand_list_a")).await;
    let course_b = create_course(&mut conn, &unique_string("cand_list_b")).await;

    for i in 0..3 {
        let status = if i == 0 {
            REWARD_STATUS_TEACHER_APPROVED.to_string()
        } else {
            REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string()
        };
        let mut candidate = new_candidate(
            if i < 2 { course_a.id } else { course_b.id },
            student.id(),
            &unique_string(&format!("list_key_{i}")),
        );
        candidate.status = status;
        reward_candidate_repository::create_candidate(&mut conn, candidate)
            .await
            .unwrap();
    }

    let all =
        reward_candidate_repository::list_candidates(&mut conn, RewardCandidateFilter::default())
            .await
            .unwrap();
    assert!(all.len() >= 3);

    let by_course = reward_candidate_repository::list_candidates(
        &mut conn,
        RewardCandidateFilter {
            course_id: Some(course_a.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(by_course.len(), 2);

    let count = reward_candidate_repository::count_candidates(
        &mut conn,
        RewardCandidateFilter {
            course_id: Some(course_a.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(count, 2);
}
