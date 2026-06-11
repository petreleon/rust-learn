#[actix_web::test]
async fn custom_course_roles_with_reward_permissions_can_submit_and_approve_candidates() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("CustomRewardRoleCourse")).await;
    let submitter = create_user_helper(&mut conn, "custom_reward_submitter").await;
    let approver = create_user_helper(&mut conn, "custom_reward_approver").await;
    let student = create_user_helper(&mut conn, "custom_reward_student").await;

    let submitter_role_id = create_custom_course_role(
        &mut conn,
        &unique_string("CURRICULUM_REWARD_OPERATOR"),
        &[Permissions::SUBMIT_COURSE_REWARD_EVENT],
    )
    .await;
    let approver_role_id = create_custom_course_role(
        &mut conn,
        &unique_string("COURSE_REWARD_CONFIRMER"),
        &[Permissions::APPROVE_STUDENT_REWARD_CANDIDATE],
    )
    .await;
    assign_course_role_id(&mut conn, submitter.id(), course.id, submitter_role_id).await;
    assign_course_role_id(&mut conn, approver.id(), course.id, approver_role_id).await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        submitter.id(),
        course.id,
        reward_request(student.id(), &unique_string("custom_role_reward")),
    )
    .await
    .expect("custom role with submit permission should create reward candidate");
    assert_eq!(candidate.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
    let submitted_audit = list_reward_audit_events(&mut conn, candidate.id)
        .await
        .expect("submitted reward audit events should load");
    assert_eq!(submitted_audit.len(), 1);
    assert_eq!(
        submitted_audit[0].event_type,
        REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED
    );
    assert_eq!(submitted_audit[0].actor_user_id, Some(submitter.id()));
    assert_eq!(
        submitted_audit[0].to_status,
        REWARD_STATUS_PENDING_TEACHER_APPROVAL
    );
    assert_eq!(candidate.submitter_user_id, submitter.id());

    let approved = decide_reward_candidate_by_teacher(
        &mut conn,
        approver.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("custom permission bundle confirmed reward".to_string()),
        },
    )
    .await
    .expect("custom role with approval permission should approve reward candidate");
    assert_eq!(approved.status, REWARD_STATUS_TEACHER_APPROVED);
    assert_eq!(approved.teacher_approver_user_id, Some(approver.id()));
}
