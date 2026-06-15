use crate::{
    amount_decision_helper::*, force_assign_organization_role::*, link_course_to_organization::*,
    reward_candidate_error::RewardCandidateError, submission_helper::*, support::*,
    teacher_decision_helper::*,
};

#[actix_web::test]
async fn teacher_submits_and_approves_then_platform_reviewer_sets_amount() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("RewardCourse")).await;
    let teacher = create_user_helper(&mut conn, "reward_teacher").await;
    let student = create_user_helper(&mut conn, "reward_student").await;
    let reviewer = create_user_helper(&mut conn, "reward_amount_reviewer").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    force_assign_platform_role(&mut conn, reviewer.id(), "MODERATOR").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let idempotency_key = unique_string("course_completion");
    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &idempotency_key),
    )
    .await
    .expect("course teacher should submit reward candidate");
    assert_eq!(candidate.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);

    let duplicate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &idempotency_key),
    )
    .await
    .expect("duplicate reward candidate submission should be idempotent");
    assert_eq!(duplicate.id, candidate.id);

    let early_amount = decide_reward_amount(
        &mut conn,
        reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: None,
        },
    )
    .await
    .expect_err("platform amount reviewer should wait for teacher approval");
    assert!(matches!(
        early_amount,
        RewardCandidateError::InvalidStatus(_)
    ));

    let teacher_approved = decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("student completed the course".to_string()),
        },
    )
    .await
    .expect("course teacher should approve reward candidate");
    assert_eq!(teacher_approved.status, REWARD_STATUS_TEACHER_APPROVED);
    assert_eq!(
        teacher_approved.teacher_approver_user_id,
        Some(teacher.id())
    );
    assert!(teacher_approved.approved_amount.is_none());

    let teacher_replay = decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("retry after transport timeout".to_string()),
        },
    )
    .await
    .expect("repeated teacher approval should be idempotent");
    assert_eq!(teacher_replay.id, teacher_approved.id);
    assert_eq!(teacher_replay.status, REWARD_STATUS_TEACHER_APPROVED);
    assert_eq!(teacher_replay.approved_amount, None);

    let amount = BigDecimal::from_str("25.50").expect("valid decimal");
    let amount_approved = decide_reward_amount(
        &mut conn,
        reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(amount.clone()),
            decision_reason: Some("amount approved by central reviewer".to_string()),
        },
    )
    .await
    .expect("platform reviewer should approve amount after teacher approval");
    assert_eq!(amount_approved.status, REWARD_STATUS_AMOUNT_APPROVED);
    assert_eq!(amount_approved.amount_reviewer_user_id, Some(reviewer.id()));
    assert_eq!(amount_approved.approved_amount, Some(amount.clone()));

    let execution_job = find_job_by_candidate(&mut conn, candidate.id)
        .await
        .expect("execution job lookup should succeed")
        .expect("amount approval should enqueue execution job");
    assert_eq!(execution_job.status, REWARD_EXECUTION_STATUS_QUEUED);

    let amount_replay = decide_reward_amount(
        &mut conn,
        reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(999)),
            decision_reason: Some("retry with stale client payload".to_string()),
        },
    )
    .await
    .expect("repeated amount approval should return the existing decision");
    assert_eq!(amount_replay.id, amount_approved.id);
    assert_eq!(amount_replay.status, REWARD_STATUS_AMOUNT_APPROVED);
    assert_eq!(amount_replay.approved_amount, Some(amount));

    let execution_job_count: i64 = reward_execution_jobs::table
        .filter(reward_execution_jobs::reward_candidate_id.eq(candidate.id))
        .count()
        .get_result(&mut conn)
        .await
        .expect("execution jobs should be countable");
    assert_eq!(
        execution_job_count, 1,
        "idempotent amount approval retry must not enqueue another execution job"
    );

    let audit = list_reward_audit_events(&mut conn, candidate.id)
        .await
        .expect("reward audit events should load");
    assert_eq!(
        audit.len(),
        3,
        "idempotent approval retries must not create duplicate audit transitions"
    );
    assert_eq!(audit[0].event_type, REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED);
    assert_eq!(audit[0].from_status, None);
    assert_eq!(audit[0].to_status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
    assert_eq!(audit[1].event_type, REWARD_AUDIT_EVENT_TEACHER_DECISION);
    assert_eq!(
        audit[1].from_status.as_deref(),
        Some(REWARD_STATUS_PENDING_TEACHER_APPROVAL)
    );
    assert_eq!(audit[1].to_status, REWARD_STATUS_TEACHER_APPROVED);
    assert_eq!(audit[1].actor_user_id, Some(teacher.id()));
    assert_eq!(audit[2].event_type, REWARD_AUDIT_EVENT_AMOUNT_DECISION);
    assert_eq!(
        audit[2].from_status.as_deref(),
        Some(REWARD_STATUS_TEACHER_APPROVED)
    );
    assert_eq!(audit[2].to_status, REWARD_STATUS_AMOUNT_APPROVED);
    assert_eq!(audit[2].actor_user_id, Some(reviewer.id()));
}
