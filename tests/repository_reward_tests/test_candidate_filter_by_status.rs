use crate::support::*;

#[actix_web::test]
async fn test_candidate_filter_by_status() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_status").await;
    let course = create_course(&mut conn, &unique_string("cand_status_course")).await;

    reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("status_a")),
    )
    .await
    .unwrap();

    let mut approved = new_candidate(course.id, student.id(), &unique_string("status_b"));
    approved.status = REWARD_STATUS_TEACHER_APPROVED.to_string();
    reward_candidate_repository::create_candidate(&mut conn, approved)
        .await
        .unwrap();

    let pending = reward_candidate_repository::list_candidates(
        &mut conn,
        RewardCandidateFilter {
            course_id: Some(course.id),
            status: Some(REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
}

#[actix_web::test]
async fn test_candidate_teacher_decision_update() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_teacher").await;
    let teacher = create_user_helper(&mut conn, "cand_teacher_actor").await;
    let course = create_course(&mut conn, &unique_string("cand_teacher_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("teacher_dec")),
    )
    .await
    .unwrap();

    let now = Utc::now();
    let updated = reward_candidate_repository::update_teacher_decision(
        &mut conn,
        candidate.id,
        teacher.id(),
        REWARD_STATUS_TEACHER_APPROVED,
        Some("looks good"),
        now,
    )
    .await
    .unwrap();

    assert_eq!(updated.status, REWARD_STATUS_TEACHER_APPROVED);
    assert_eq!(updated.teacher_approver_user_id, Some(teacher.id()));
    assert_eq!(
        updated.teacher_decision_reason,
        Some("looks good".to_string())
    );
    assert!(updated.teacher_decided_at.is_some());
}

#[actix_web::test]
async fn test_candidate_amount_decision_update() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_amount").await;
    let reviewer = create_user_helper(&mut conn, "cand_amount_rev").await;
    let course = create_course(&mut conn, &unique_string("cand_amount_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("amount_dec")),
    )
    .await
    .unwrap();

    let amount = BigDecimal::from(100);
    let now = Utc::now();
    let updated = reward_candidate_repository::update_amount_decision(
        &mut conn,
        candidate.id,
        reviewer.id(),
        "amount_approved",
        Some(amount.clone()),
        Some("approved amount"),
        now,
    )
    .await
    .unwrap();

    assert_eq!(updated.status, "amount_approved");
    assert_eq!(updated.amount_reviewer_user_id, Some(reviewer.id()));
    assert_eq!(updated.approved_amount, Some(amount));
    assert!(updated.amount_decided_at.is_some());
}
