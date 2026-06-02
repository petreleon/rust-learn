use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{
    courses, courses_organizations, organizations, reward_policies, users,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::{
    REWARD_EVENT_COURSE_COMPLETION, REWARD_EVENT_MANUAL_COMPLETION, REWARD_SOURCE_ORGANIZATION,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TEACHER_APPROVED,
};
use rust_learn::models::reward_execution_job::REWARD_EXECUTION_STATUS_QUEUED;
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::reward_execution_job_repository::find_job_by_candidate;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_candidate_service::{
    decide_reward_amount, decide_reward_candidate_by_teacher, submit_course_reward_candidate,
    submit_organization_reward_candidate, RewardAmountDecisionRequest, RewardCandidateError,
    SubmitRewardCandidateRequest, TeacherRewardCandidateDecisionRequest,
};
use serde_json::json;
use std::str::FromStr;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let user = create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");
    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(true))
        .execute(conn)
        .await
        .expect("failed to verify user email");
    user
}

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: name.to_string(),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

async fn link_course_to_organization(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course to organization");
}

async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role not found");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn force_assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role not found");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

async fn create_active_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: event_type.to_string(),
            version: 1,
            token_amount: BigDecimal::from(10),
            multiplier: BigDecimal::from(1),
            max_payout: Some(BigDecimal::from(100)),
            cooldown_seconds: 0,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .execute(conn)
        .await
        .expect("failed to create reward policy");
}

fn reward_request(student_user_id: i32, idempotency_key: &str) -> SubmitRewardCandidateRequest {
    SubmitRewardCandidateRequest {
        student_user_id,
        event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
        idempotency_key: Some(idempotency_key.to_string()),
        evidence: Some(json!({ "completion_percentage": 100 })),
    }
}

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
    assert_eq!(amount_approved.approved_amount, Some(amount));

    let execution_job = find_job_by_candidate(&mut conn, candidate.id)
        .await
        .expect("execution job lookup should succeed")
        .expect("amount approval should enqueue execution job");
    assert_eq!(execution_job.status, REWARD_EXECUTION_STATUS_QUEUED);
}

#[actix_web::test]
async fn platform_amount_permission_cannot_submit_or_approve_candidate() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("RewardBoundaryCourse")).await;
    let teacher = create_user_helper(&mut conn, "reward_boundary_teacher").await;
    let student = create_user_helper(&mut conn, "reward_boundary_student").await;
    let reviewer = create_user_helper(&mut conn, "reward_boundary_reviewer").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    force_assign_platform_role(&mut conn, reviewer.id(), "MODERATOR").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let denied_submit = submit_course_reward_candidate(
        &mut conn,
        reviewer.id(),
        course.id,
        reward_request(student.id(), &unique_string("denied_reward")),
    )
    .await
    .expect_err("platform amount reviewer must not submit course reward candidates");
    assert!(matches!(
        denied_submit,
        RewardCandidateError::PermissionDenied(_)
    ));

    let denied_student_submit = submit_course_reward_candidate(
        &mut conn,
        student.id(),
        course.id,
        reward_request(student.id(), &unique_string("student_self_reward")),
    )
    .await
    .expect_err("student activity evidence must not authorize self-submission");
    assert!(matches!(
        denied_student_submit,
        RewardCandidateError::PermissionDenied(_)
    ));

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("teacher_reward")),
    )
    .await
    .expect("teacher should submit reward candidate");

    let denied_teacher_decision = decide_reward_candidate_by_teacher(
        &mut conn,
        reviewer.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect_err("platform amount reviewer must not approve reward candidates");
    assert!(matches!(
        denied_teacher_decision,
        RewardCandidateError::PermissionDenied(_)
    ));

    decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect("teacher approval should succeed");

    let teacher_amount_decision = decide_reward_amount(
        &mut conn,
        teacher.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(12)),
            decision_reason: None,
        },
    )
    .await
    .expect_err("course teacher must not set platform reward amount");
    assert!(matches!(
        teacher_amount_decision,
        RewardCandidateError::PermissionDenied(_)
    ));
}

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
    assert_eq!(candidate.source_scope, REWARD_SOURCE_ORGANIZATION);
    assert_eq!(candidate.source_organization_id, Some(organization.id));
    assert_eq!(candidate.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);

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
    assert_eq!(approved.status, REWARD_STATUS_TEACHER_APPROVED);
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
