use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{
    courses, courses_organizations, organizations, reward_policies, users,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::delegated_permission::{
    GrantDelegatedPermissionRequest, DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION,
    DELEGATED_SCOPE_PLATFORM,
};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::{
    REWARD_EVENT_COURSE_COMPLETION, REWARD_STATUS_AMOUNT_APPROVED,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::role::{CourseRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::course_repository::user_permission_course_request;
use rust_learn::repositories::delegated_permission_repository::find_delegated_permission;
use rust_learn::repositories::organization_repository::user_permission_organization_request;
use rust_learn::repositories::platform_repository::user_permission_platform_request;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::delegated_permission_service::{
    grant_delegated_permission, revoke_delegated_permission,
};
use rust_learn::services::reward_candidate_service::{
    decide_reward_amount, decide_reward_candidate_by_teacher, submit_course_reward_candidate,
    submit_organization_reward_candidate, RewardAmountDecisionRequest, RewardCandidateError,
    SubmitRewardCandidateRequest, TeacherRewardCandidateDecisionRequest,
};
use serde_json::json;

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

async fn create_active_course_reward_policy(conn: &mut AsyncPgConnection, course_id: i32) {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
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
async fn delegated_course_permission_submits_candidate_without_course_role() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("DelegatedCourse")).await;
    let other_course = create_course(&mut conn, &unique_string("DelegatedOtherCourse")).await;
    let admin = create_user_helper(&mut conn, "delegating_admin").await;
    let operator = create_user_helper(&mut conn, "delegated_operator").await;
    let student = create_user_helper(&mut conn, "delegated_student").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    create_active_course_reward_policy(&mut conn, course.id).await;

    let denied = submit_course_reward_candidate(
        &mut conn,
        operator.id(),
        course.id,
        reward_request(student.id(), &unique_string("delegated_denied")),
    )
    .await
    .expect_err("operator without role or delegation must be denied");
    assert!(matches!(denied, RewardCandidateError::PermissionDenied(_)));

    grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
            scope_type: DELEGATED_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            reason: Some("central office reward operations".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate course reward submission");

    assert!(user_permission_course_request(
        &mut conn,
        operator.id(),
        course.id,
        &Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("course permission lookup should succeed"));
    assert!(!user_permission_course_request(
        &mut conn,
        operator.id(),
        other_course.id,
        &Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("other course permission lookup should succeed"));

    let candidate = submit_course_reward_candidate(
        &mut conn,
        operator.id(),
        course.id,
        reward_request(student.id(), &unique_string("delegated_course_submit")),
    )
    .await
    .expect("delegated operator should submit candidate");
    assert_eq!(candidate.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
}

#[actix_web::test]
async fn delegated_permission_grant_requires_platform_delegate_permission() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("DelegatedDeniedCourse")).await;
    let grantor = create_user_helper(&mut conn, "delegating_denied_grantor").await;
    let operator = create_user_helper(&mut conn, "delegating_denied_operator").await;

    let denied = grant_delegated_permission(
        &mut conn,
        grantor.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
            scope_type: DELEGATED_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            reason: Some("should not grant without platform permission".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect_err("user without delegation authority must be denied");

    assert!(matches!(
        denied,
        rust_learn::services::delegated_permission_service::DelegatedPermissionError::PermissionDenied(permission)
            if permission == Permissions::DELEGATE_REWARD_APPROVAL.to_string()
    ));
}

#[actix_web::test]
async fn delegated_organization_permission_submits_for_attached_course_only() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("DelegatedOrg")).await;
    let other_organization =
        create_organization(&mut conn, &unique_string("DelegatedOtherOrg")).await;
    let course = create_course(&mut conn, &unique_string("DelegatedOrgCourse")).await;
    let admin = create_user_helper(&mut conn, "delegating_org_admin").await;
    let operator = create_user_helper(&mut conn, "delegated_org_operator").await;
    let student = create_user_helper(&mut conn, "delegated_org_student").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;
    create_active_course_reward_policy(&mut conn, course.id).await;

    grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string(),
            scope_type: DELEGATED_SCOPE_ORGANIZATION.to_string(),
            organization_id: Some(organization.id),
            course_id: None,
            reason: Some("central org reward operations".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate organization reward submission");

    assert!(user_permission_organization_request(
        &mut conn,
        operator.id(),
        organization.id,
        &Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("organization permission lookup should succeed"));
    assert!(!user_permission_organization_request(
        &mut conn,
        operator.id(),
        other_organization.id,
        &Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("other organization permission lookup should succeed"));

    let candidate = submit_organization_reward_candidate(
        &mut conn,
        operator.id(),
        organization.id,
        course.id,
        reward_request(student.id(), &unique_string("delegated_org_submit")),
    )
    .await
    .expect("delegated organization operator should submit attached course reward");
    assert_eq!(candidate.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
}

#[actix_web::test]
async fn delegated_platform_amount_reviewer_can_set_amount_after_teacher_approval() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("DelegatedAmountCourse")).await;
    let admin = create_user_helper(&mut conn, "delegating_amount_admin").await;
    let teacher = create_user_helper(&mut conn, "delegated_amount_teacher").await;
    let operator = create_user_helper(&mut conn, "delegated_amount_operator").await;
    let student = create_user_helper(&mut conn, "delegated_amount_student").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    create_active_course_reward_policy(&mut conn, course.id).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("delegated_amount_candidate")),
    )
    .await
    .expect("teacher should submit candidate");
    let approved = decide_reward_candidate_by_teacher(
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
    .expect("teacher should approve candidate");
    assert_eq!(approved.status, REWARD_STATUS_TEACHER_APPROVED);

    let denied = decide_reward_amount(
        &mut conn,
        operator.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: None,
        },
    )
    .await
    .expect_err("operator without platform delegation must be denied");
    assert!(matches!(denied, RewardCandidateError::PermissionDenied(_)));

    grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::APPROVE_REWARD_AMOUNT.to_string(),
            scope_type: DELEGATED_SCOPE_PLATFORM.to_string(),
            organization_id: None,
            course_id: None,
            reason: Some("central reward amount review".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate platform amount review");

    assert!(user_permission_platform_request(
        &mut conn,
        operator.id(),
        &Permissions::APPROVE_REWARD_AMOUNT.to_string(),
    )
    .await
    .expect("platform permission lookup should succeed"));

    let amount_approved = decide_reward_amount(
        &mut conn,
        operator.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: None,
        },
    )
    .await
    .expect("delegated platform reviewer should approve amount");
    assert_eq!(amount_approved.status, REWARD_STATUS_AMOUNT_APPROVED);
}

#[actix_web::test]
async fn revoked_delegation_no_longer_authorizes_permission() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("DelegatedRevokeCourse")).await;
    let admin = create_user_helper(&mut conn, "delegating_revoke_admin").await;
    let operator = create_user_helper(&mut conn, "delegated_revoke_operator").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;

    let delegation = grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
            scope_type: DELEGATED_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            reason: Some("temporary course approval coverage".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate course approval");

    assert!(user_permission_course_request(
        &mut conn,
        operator.id(),
        course.id,
        &Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
    )
    .await
    .expect("course permission lookup should succeed"));

    revoke_delegated_permission(
        &mut conn,
        admin.id(),
        delegation.id,
        Some("coverage ended".to_string()),
    )
    .await
    .expect("admin should revoke delegated permission");

    assert!(!user_permission_course_request(
        &mut conn,
        operator.id(),
        course.id,
        &Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
    )
    .await
    .expect("course permission lookup after revoke should succeed"));

    let stored = find_delegated_permission(&mut conn, delegation.id)
        .await
        .expect("delegation record should remain after revoke");
    assert_eq!(stored.revoked_by_user_id, Some(admin.id()));
    assert!(stored.revoked_at.is_some());
}
