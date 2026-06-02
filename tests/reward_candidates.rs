use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{
    course_roles, courses, courses_organizations, organizations, platform_roles, reward_policies,
    role_permission_course, role_permission_platform, users,
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
use rust_learn::models::reward_fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::reward_candidate_repository::find_candidate;
use rust_learn::repositories::reward_execution_job_repository::find_job_by_candidate;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_candidate_service::{
    decide_reward_amount, decide_reward_candidate_by_teacher, submit_course_reward_candidate,
    submit_organization_reward_candidate, RewardAmountDecisionRequest, RewardCandidateError,
    SubmitRewardCandidateRequest, TeacherRewardCandidateDecisionRequest,
};
use rust_learn::services::reward_fraud_block_service::{
    create_reward_fraud_block, revoke_reward_fraud_block, RewardFraudBlockRequest,
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

async fn create_custom_course_role(
    conn: &mut AsyncPgConnection,
    role_name: &str,
    permissions: &[Permissions],
) -> i32 {
    let role_id = diesel::insert_into(course_roles::table)
        .values((
            course_roles::name.eq(role_name),
            course_roles::description.eq(Some("test-only permission bundle".to_string())),
        ))
        .returning(course_roles::id)
        .get_result::<i32>(conn)
        .await
        .expect("failed to create custom course role");

    for permission in permissions {
        diesel::insert_into(role_permission_course::table)
            .values((
                role_permission_course::course_id.eq(None::<i32>),
                role_permission_course::course_role_id.eq(Some(role_id)),
                role_permission_course::permission.eq(permission.to_string()),
            ))
            .execute(conn)
            .await
            .expect("failed to assign custom course role permission");
    }

    role_id
}

async fn assign_course_role_id(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_id: i32,
) {
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign custom course role");
}

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role not found");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn create_custom_platform_role(
    conn: &mut AsyncPgConnection,
    role_name: &str,
    permissions: &[Permissions],
) -> i32 {
    let role_id = diesel::insert_into(platform_roles::table)
        .values((
            platform_roles::name.eq(role_name),
            platform_roles::description
                .eq(Some("test-only platform permission bundle".to_string())),
        ))
        .returning(platform_roles::id)
        .get_result::<i32>(conn)
        .await
        .expect("failed to create custom platform role");

    for permission in permissions {
        diesel::insert_into(role_permission_platform::table)
            .values((
                role_permission_platform::platform_role_id.eq(Some(role_id)),
                role_permission_platform::permission.eq(permission.to_string()),
            ))
            .execute(conn)
            .await
            .expect("failed to assign custom platform role permission");
    }

    role_id
}

async fn assign_platform_role_id(conn: &mut AsyncPgConnection, user_id: i32, role_id: i32) {
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign custom platform role");
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
) -> i64 {
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
        .returning(reward_policies::id)
        .get_result(conn)
        .await
        .expect("failed to create reward policy")
}

fn reward_request(student_user_id: i32, idempotency_key: &str) -> SubmitRewardCandidateRequest {
    SubmitRewardCandidateRequest {
        student_user_id,
        event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
        idempotency_key: Some(idempotency_key.to_string()),
        evidence: Some(json!({ "completion_percentage": 100 })),
    }
}

fn teacher_fraud_block_request(teacher_user_id: i32) -> RewardFraudBlockRequest {
    RewardFraudBlockRequest {
        scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
        teacher_user_id: Some(teacher_user_id),
        organization_id: None,
        course_id: None,
        reward_policy_id: None,
        reason: "teacher reward activity paused".to_string(),
        evidence_reference: Some("case://reward-candidate-flow".to_string()),
        expires_at: None,
    }
}

#[derive(Clone, Copy)]
enum FraudBlockScopeUnderTest {
    Organization,
    Course,
    RewardPolicy,
}

fn scoped_fraud_block_request(
    scope: FraudBlockScopeUnderTest,
    organization_id: i32,
    course_id: i32,
    reward_policy_id: i64,
) -> RewardFraudBlockRequest {
    match scope {
        FraudBlockScopeUnderTest::Organization => RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION.to_string(),
            teacher_user_id: None,
            organization_id: Some(organization_id),
            course_id: None,
            reward_policy_id: None,
            reason: "organization reward activity paused".to_string(),
            evidence_reference: Some("case://organization-reward-block".to_string()),
            expires_at: None,
        },
        FraudBlockScopeUnderTest::Course => RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string(),
            teacher_user_id: None,
            organization_id: None,
            course_id: Some(course_id),
            reward_policy_id: None,
            reason: "course reward activity paused".to_string(),
            evidence_reference: Some("case://course-reward-block".to_string()),
            expires_at: None,
        },
        FraudBlockScopeUnderTest::RewardPolicy => RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY.to_string(),
            teacher_user_id: None,
            organization_id: None,
            course_id: None,
            reward_policy_id: Some(reward_policy_id),
            reason: "reward policy activity paused".to_string(),
            evidence_reference: Some("case://policy-reward-block".to_string()),
            expires_at: None,
        },
    }
}

fn expected_block_message(scope: FraudBlockScopeUnderTest) -> &'static str {
    match scope {
        FraudBlockScopeUnderTest::Organization => "organization reward activity is blocked",
        FraudBlockScopeUnderTest::Course => "course reward activity is blocked",
        FraudBlockScopeUnderTest::RewardPolicy => "reward policy activity is blocked",
    }
}

async fn assert_scoped_fraud_block_pauses_reward_activity(scope: FraudBlockScopeUnderTest) {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("ScopedBlockOrg")).await;
    let course = create_course(&mut conn, &unique_string("ScopedBlockCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;
    let teacher = create_user_helper(&mut conn, "scoped_block_teacher").await;
    let student = create_user_helper(&mut conn, "scoped_block_student").await;
    let second_student = create_user_helper(&mut conn, "scoped_block_student_two").await;
    let reviewer = create_user_helper(&mut conn, "scoped_block_reviewer").await;
    let admin = create_user_helper(&mut conn, "scoped_block_admin").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    force_assign_course_role(&mut conn, second_student.id(), course.id, "STUDENT").await;
    force_assign_platform_role(&mut conn, reviewer.id(), "MODERATOR").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    let reward_policy_id =
        create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION)
            .await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("scoped_block_candidate")),
    )
    .await
    .expect("teacher should submit before scoped fraud block is active");

    let block_request =
        scoped_fraud_block_request(scope, organization.id, course.id, reward_policy_id);
    let active_block = create_reward_fraud_block(&mut conn, admin.id(), block_request.clone())
        .await
        .expect("admin should create scoped fraud block");

    let denied_submission = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(
            second_student.id(),
            &unique_string("scoped_block_submission"),
        ),
    )
    .await
    .expect_err("active scoped fraud block should prevent new reward submission");
    assert!(matches!(
        denied_submission,
        RewardCandidateError::InvalidStatus(message)
            if message.contains(expected_block_message(scope))
    ));

    let denied_teacher_approval = decide_reward_candidate_by_teacher(
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
    .expect_err("active scoped fraud block should prevent teacher approval");
    assert!(matches!(
        denied_teacher_approval,
        RewardCandidateError::InvalidStatus(message)
            if message.contains(expected_block_message(scope))
    ));

    revoke_reward_fraud_block(&mut conn, admin.id(), active_block.id)
        .await
        .expect("admin should revoke scoped fraud block");

    decide_reward_candidate_by_teacher(
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
    .expect("teacher approval should resume after scoped block revocation");

    create_reward_fraud_block(&mut conn, admin.id(), block_request)
        .await
        .expect("admin should recreate scoped fraud block");

    let denied_amount = decide_reward_amount(
        &mut conn,
        reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: Some("amount approved by central reviewer".to_string()),
        },
    )
    .await
    .expect_err("active scoped fraud block should prevent amount approval");
    assert!(matches!(
        denied_amount,
        RewardCandidateError::InvalidStatus(message)
            if message.contains(expected_block_message(scope))
    ));
}

#[actix_web::test]
async fn organization_course_and_policy_fraud_blocks_pause_reward_activity() {
    for scope in [
        FraudBlockScopeUnderTest::Organization,
        FraudBlockScopeUnderTest::Course,
        FraudBlockScopeUnderTest::RewardPolicy,
    ] {
        assert_scoped_fraud_block_pauses_reward_activity(scope).await;
    }
}

#[actix_web::test]
async fn fraud_block_permission_cannot_approve_candidates_or_amounts() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("FraudPermissionBoundaryCourse")).await;
    let teacher = create_user_helper(&mut conn, "fraud_permission_teacher").await;
    let student = create_user_helper(&mut conn, "fraud_permission_student").await;
    let fraud_operator = create_user_helper(&mut conn, "fraud_permission_operator").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let fraud_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("FRAUD_BLOCK_ONLY_OPERATOR"),
        &[Permissions::MANAGE_REWARD_FRAUD_BLOCKS],
    )
    .await;
    assign_platform_role_id(&mut conn, fraud_operator.id(), fraud_role_id).await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let block = create_reward_fraud_block(
        &mut conn,
        fraud_operator.id(),
        teacher_fraud_block_request(teacher.id()),
    )
    .await
    .expect("fraud-block permission should create teacher reward block");
    revoke_reward_fraud_block(&mut conn, fraud_operator.id(), block.id)
        .await
        .expect("fraud-block permission should revoke teacher reward block");

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("fraud_permission_candidate")),
    )
    .await
    .expect("teacher should submit after fraud block revocation");

    let denied_candidate_approval = decide_reward_candidate_by_teacher(
        &mut conn,
        fraud_operator.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("fraud operator attempted candidate approval".to_string()),
        },
    )
    .await
    .expect_err("fraud-block permission must not approve reward candidates");
    assert!(matches!(
        denied_candidate_approval,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()
    ));

    let denied_candidate_rejection = decide_reward_candidate_by_teacher(
        &mut conn,
        fraud_operator.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "rejected".to_string(),
            decision_reason: Some("fraud operator attempted candidate rejection".to_string()),
        },
    )
    .await
    .expect_err("fraud-block permission must not reject reward candidates");
    assert!(matches!(
        denied_candidate_rejection,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()
    ));

    decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("teacher confirmed reward candidate".to_string()),
        },
    )
    .await
    .expect("teacher should approve candidate");

    let denied_amount = decide_reward_amount(
        &mut conn,
        fraud_operator.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(15)),
            decision_reason: Some("fraud operator attempted amount approval".to_string()),
        },
    )
    .await
    .expect_err("fraud-block permission must not set payout amount");
    assert!(matches!(
        denied_amount,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_REWARD_AMOUNT.to_string()
    ));

    let execution_job = find_job_by_candidate(&mut conn, candidate.id)
        .await
        .expect("execution job lookup should succeed");
    assert!(
        execution_job.is_none(),
        "fraud-block permission must not enqueue reward execution"
    );
}

#[actix_web::test]
async fn reward_amount_approval_requires_platform_scoped_permission() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("PlatformAmountScopeCourse")).await;
    let teacher = create_user_helper(&mut conn, "platform_amount_teacher").await;
    let student = create_user_helper(&mut conn, "platform_amount_student").await;
    let platform_reviewer = create_user_helper(&mut conn, "platform_amount_reviewer").await;
    let course_scoped_reviewer = create_user_helper(&mut conn, "course_amount_reviewer").await;

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let platform_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("CENTRAL_AMOUNT_REVIEWER"),
        &[Permissions::APPROVE_REWARD_AMOUNT],
    )
    .await;
    let course_role_id = create_custom_course_role(
        &mut conn,
        &unique_string("COURSE_AMOUNT_REVIEWER"),
        &[Permissions::APPROVE_REWARD_AMOUNT],
    )
    .await;
    assign_platform_role_id(&mut conn, platform_reviewer.id(), platform_role_id).await;
    assign_course_role_id(
        &mut conn,
        course_scoped_reviewer.id(),
        course.id,
        course_role_id,
    )
    .await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("platform_amount_scope")),
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
            decision_reason: Some("course approval complete".to_string()),
        },
    )
    .await
    .expect("teacher should approve candidate before amount review");

    let course_scoped_denied = decide_reward_amount(
        &mut conn,
        course_scoped_reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(20)),
            decision_reason: Some("course-scoped amount attempt".to_string()),
        },
    )
    .await
    .expect_err("course-scoped amount permission must not approve platform amount");
    assert!(matches!(
        course_scoped_denied,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_REWARD_AMOUNT.to_string()
    ));

    let approved = decide_reward_amount(
        &mut conn,
        platform_reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(20)),
            decision_reason: Some("platform amount approved".to_string()),
        },
    )
    .await
    .expect("platform-scoped amount permission should approve reward amount");
    assert_eq!(approved.status, REWARD_STATUS_AMOUNT_APPROVED);
    assert_eq!(
        approved.amount_reviewer_user_id,
        Some(platform_reviewer.id())
    );
}

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
async fn teacher_fraud_block_pauses_submission_teacher_approval_and_amount_approval() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("BlockedRewardFlowCourse")).await;
    let teacher = create_user_helper(&mut conn, "blocked_flow_teacher").await;
    let student = create_user_helper(&mut conn, "blocked_flow_student").await;
    let second_student = create_user_helper(&mut conn, "blocked_flow_student_two").await;
    let reviewer = create_user_helper(&mut conn, "blocked_flow_reviewer").await;
    let admin = create_user_helper(&mut conn, "blocked_flow_admin").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    force_assign_course_role(&mut conn, second_student.id(), course.id, "STUDENT").await;
    force_assign_platform_role(&mut conn, reviewer.id(), "MODERATOR").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("blocked_flow_candidate")),
    )
    .await
    .expect("teacher should submit before fraud block is active");

    let teacher_block = create_reward_fraud_block(
        &mut conn,
        admin.id(),
        teacher_fraud_block_request(teacher.id()),
    )
    .await
    .expect("admin should block teacher reward activity");

    let denied_submission = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(
            second_student.id(),
            &unique_string("blocked_flow_submission"),
        ),
    )
    .await
    .expect_err("active teacher block should prevent new reward submission");
    assert!(matches!(
        denied_submission,
        RewardCandidateError::InvalidStatus(message)
            if message.contains("teacher reward activity is blocked")
    ));

    let denied_teacher_approval = decide_reward_candidate_by_teacher(
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
    .expect_err("active teacher block should prevent teacher approval");
    assert!(matches!(
        denied_teacher_approval,
        RewardCandidateError::InvalidStatus(message)
            if message.contains("teacher reward activity is blocked")
    ));
    let still_pending = find_candidate(&mut conn, candidate.id)
        .await
        .expect("candidate should remain readable after blocked teacher decision");
    assert_eq!(still_pending.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
    assert!(still_pending.teacher_approver_user_id.is_none());

    revoke_reward_fraud_block(&mut conn, admin.id(), teacher_block.id)
        .await
        .expect("admin should revoke teacher fraud block");

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
    .expect("teacher approval should resume after revocation");
    assert_eq!(teacher_approved.status, REWARD_STATUS_TEACHER_APPROVED);

    create_reward_fraud_block(
        &mut conn,
        admin.id(),
        teacher_fraud_block_request(teacher.id()),
    )
    .await
    .expect("admin should create a new active teacher fraud block");

    let denied_amount = decide_reward_amount(
        &mut conn,
        reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: Some("amount approved by central reviewer".to_string()),
        },
    )
    .await
    .expect_err("active teacher block should prevent amount approval");
    assert!(matches!(
        denied_amount,
        RewardCandidateError::InvalidStatus(message)
            if message.contains("teacher reward activity is blocked")
    ));
    let still_teacher_approved = find_candidate(&mut conn, candidate.id)
        .await
        .expect("candidate should remain readable after blocked amount decision");
    assert_eq!(
        still_teacher_approved.status,
        REWARD_STATUS_TEACHER_APPROVED
    );
    assert!(still_teacher_approved.amount_reviewer_user_id.is_none());
    assert!(still_teacher_approved.approved_amount.is_none());

    let execution_job = find_job_by_candidate(&mut conn, candidate.id)
        .await
        .expect("execution job lookup should succeed");
    assert!(
        execution_job.is_none(),
        "fraud block must not enqueue reward execution by deciding a candidate"
    );
}

#[actix_web::test]
async fn platform_amount_permission_cannot_submit_or_approve_candidate() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("RewardBoundaryOrg")).await;
    let course = create_course(&mut conn, &unique_string("RewardBoundaryCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;
    let teacher = create_user_helper(&mut conn, "reward_boundary_teacher").await;
    let student = create_user_helper(&mut conn, "reward_boundary_student").await;
    let reviewer = create_user_helper(&mut conn, "reward_boundary_reviewer").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let amount_only_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("AMOUNT_ONLY_REVIEWER"),
        &[Permissions::APPROVE_REWARD_AMOUNT],
    )
    .await;
    assign_platform_role_id(&mut conn, reviewer.id(), amount_only_role_id).await;
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

    let denied_org_submit = submit_organization_reward_candidate(
        &mut conn,
        reviewer.id(),
        organization.id,
        course.id,
        reward_request(student.id(), &unique_string("denied_org_reward")),
    )
    .await
    .expect_err("platform amount reviewer must not submit organization reward candidates");
    assert!(matches!(
        denied_org_submit,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string()
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

    let denied_teacher_rejection = decide_reward_candidate_by_teacher(
        &mut conn,
        reviewer.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "rejected".to_string(),
            decision_reason: Some("amount reviewer attempted candidate rejection".to_string()),
        },
    )
    .await
    .expect_err("platform amount reviewer must not reject reward candidates");
    assert!(matches!(
        denied_teacher_rejection,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()
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
