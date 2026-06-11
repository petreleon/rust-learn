#[actix_web::test]
async fn teacher_named_course_role_without_reward_permissions_cannot_submit_or_approve() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("TeacherNameNoRewardCourse")).await;
    let submitter = create_user_helper(&mut conn, "no_reward_submitter").await;
    let teacher_named_user = create_user_helper(&mut conn, "no_reward_teacher_named").await;
    let student = create_user_helper(&mut conn, "no_reward_student").await;

    let submitter_role_id = create_custom_course_role(
        &mut conn,
        &unique_string("REWARD_SUBMIT_ONLY"),
        &[Permissions::SUBMIT_COURSE_REWARD_EVENT],
    )
    .await;
    let teacher_named_role_id = create_custom_course_role(
        &mut conn,
        &unique_string("TEACHER_WITHOUT_REWARD_PERMISSIONS"),
        &[],
    )
    .await;
    assign_course_role_id(&mut conn, submitter.id(), course.id, submitter_role_id).await;
    assign_course_role_id(
        &mut conn,
        teacher_named_user.id(),
        course.id,
        teacher_named_role_id,
    )
    .await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let denied_submit = submit_course_reward_candidate(
        &mut conn,
        teacher_named_user.id(),
        course.id,
        reward_request(student.id(), &unique_string("teacher_name_denied_submit")),
    )
    .await
    .expect_err("teacher-like role name without submit permission should be denied");
    assert!(matches!(
        denied_submit,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string()
    ));

    let candidate = submit_course_reward_candidate(
        &mut conn,
        submitter.id(),
        course.id,
        reward_request(student.id(), &unique_string("submitter_reward")),
    )
    .await
    .expect("submitter permission should create candidate for approval denial test");

    let denied_approval = decide_reward_candidate_by_teacher(
        &mut conn,
        teacher_named_user.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect_err("teacher-like role name without approval permission should be denied");
    assert!(matches!(
        denied_approval,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()
    ));
}
