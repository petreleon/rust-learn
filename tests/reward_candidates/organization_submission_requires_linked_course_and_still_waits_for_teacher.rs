use crate::{
    force_assign_organization_role::*, link_course_to_organization::*,
    reward_candidate_error::RewardCandidateError, submission_helper::*, support::*,
    teacher_decision_helper::*,
};
use rust_learn::domain::rewards::candidate::source::RewardCandidateSourceScope;
use rust_learn::domain::rewards::candidate::status::RewardCandidateStatus;

#[actix_web::test]
async fn organization_submission_requires_linked_course_and_still_waits_for_teacher() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("RewardOrg")).await;
    let course = create_course(&mut conn, &unique_string("RewardOrgCourse")).await;
    let unlinked_course = create_course(&mut conn, &unique_string("RewardUnlinkedCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;

    let org_admin = create_user_helper(&mut conn, "reward_org_admin").await;
    let teacher = create_user_helper(&mut conn, "reward_org_teacher").await;
    let student = create_user_helper(&mut conn, "reward_org_student").await;
    force_assign_organization_role(&mut conn, org_admin.id(), organization.id, "ADMIN").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_MANUAL_COMPLETION).await;

    let denied_unlinked = submit_organization_reward_candidate(
        &mut conn,
        org_admin.id(),
        organization.id,
        unlinked_course.id,
        reward_request(student.id(), &unique_string("unlinked_reward")),
    )
    .await
    .expect_err("organization submission should be scoped to linked courses");
    assert!(matches!(
        denied_unlinked,
        RewardCandidateError::InvalidInput(_)
    ));

    let candidate = submit_organization_reward_candidate(
        &mut conn,
        org_admin.id(),
        organization.id,
        course.id,
        SubmitRewardCandidateRequest {
            student_user_id: student.id(),
            event_type: REWARD_EVENT_MANUAL_COMPLETION.to_string(),
            idempotency_key: Some(unique_string("org_reward")),
            evidence: Some(json!({ "source": "organization dashboard" })),
        },
    )
    .await
    .expect("organization admin should submit linked course reward candidate");
    assert_eq!(
        candidate.source_scope,
        RewardCandidateSourceScope::Organization
    );
    assert_eq!(candidate.source_organization_id, Some(organization.id));
    assert_eq!(
        candidate.status,
        RewardCandidateStatus::PendingTeacherApproval
    );

    let approved = decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("teacher confirmed organization submission".to_string()),
        },
    )
    .await
    .expect("teacher approval should still be required after organization submission");
    assert_eq!(approved.status, RewardCandidateStatus::TeacherApproved);
}

#[actix_web::test]
async fn reward_candidate_requires_completion_evidence_threshold() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("RewardEvidenceCourse")).await;
    let teacher = create_user_helper(&mut conn, "reward_evidence_teacher").await;
    let student = create_user_helper(&mut conn, "reward_evidence_student").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let denied = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        SubmitRewardCandidateRequest {
            student_user_id: student.id(),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: Some(unique_string("low_completion_reward")),
            evidence: Some(json!({ "completion_percentage": 80 })),
        },
    )
    .await
    .expect_err("course completion rewards should require full completion evidence");

    assert!(matches!(denied, RewardCandidateError::InvalidInput(_)));
}
