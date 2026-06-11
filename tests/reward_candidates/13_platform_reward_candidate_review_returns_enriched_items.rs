#[actix_web::test]
async fn platform_reward_candidate_review_returns_enriched_items() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("PlatformReviewCourse")).await;
    let teacher = create_user_helper(&mut conn, "platform_review_teacher").await;
    let student = create_user_helper(&mut conn, "platform_review_student").await;
    let reviewer = create_user_helper(&mut conn, "platform_review_reviewer").await;
    let admin = create_user_helper(&mut conn, "platform_review_admin").await;

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let reviewer_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("PLATFORM_REVIEWER"),
        &[
            Permissions::VIEW_REWARD_AUDIT,
            Permissions::APPROVE_REWARD_AMOUNT,
        ],
    )
    .await;
    assign_platform_role_id(&mut conn, reviewer.id(), reviewer_role_id).await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("platform_review_candidate")),
    )
    .await
    .expect("teacher should submit reward candidate");
    decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("teacher approved for platform review".to_string()),
        },
    )
    .await
    .expect("teacher should approve candidate");

    let review = list_platform_reward_candidates(
        &mut conn,
        reviewer.id(),
        PlatformRewardCandidatesRequest {
            status: Some(REWARD_STATUS_TEACHER_APPROVED.to_string()),
            search: None,
            limit: Some(10),
            offset: Some(0),
        },
    )
    .await
    .expect("platform reviewer should see enriched candidate list");

    assert!(review.candidates.iter().any(|c| c.id == candidate.id));
    let found = review
        .candidates
        .iter()
        .find(|c| c.id == candidate.id)
        .unwrap();
    assert_eq!(found.student.id, student.id());
    assert!(!found.student.name.is_empty());
    assert!(!found.student.email.is_empty());
    assert_eq!(found.course.id, course.id);
    assert!(!found.course.title.is_empty());
    assert_eq!(found.status, REWARD_STATUS_TEACHER_APPROVED);
    assert_eq!(
        found.teacher_approver.as_ref().map(|u| u.id),
        Some(teacher.id())
    );
    assert_eq!(
        found.teacher_decision_reason.as_deref(),
        Some("teacher approved for platform review")
    );
    assert_eq!(found.submitter.id, teacher.id());
    assert!(review.operator_permissions.can_view_candidates);
    assert!(review.operator_permissions.can_approve_amount);

    let no_permission = list_platform_reward_candidates(
        &mut conn,
        student.id(),
        PlatformRewardCandidatesRequest::default(),
    )
    .await
    .expect_err("student must not view platform review queue");
    assert!(matches!(
        no_permission,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::VIEW_REWARD_AUDIT.to_string()
    ));
}
