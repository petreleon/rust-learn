use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, courses_organizations, organizations};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::course_join_request::{
    COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_PENDING,
};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::course_repository::user_permission_course_request;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::course_enrollment_service::{
    decide_course_join_request, request_course_join, CourseEnrollmentError,
    CourseJoinDecisionRequest,
};

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
    create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user")
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

#[actix_web::test]
async fn organization_student_can_request_and_course_teacher_can_approve_join() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("JoinOrg")).await;
    let course = create_course(&mut conn, &unique_string("JoinCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;

    let student = create_user_helper(&mut conn, "join_student").await;
    let teacher = create_user_helper(&mut conn, "join_teacher").await;
    force_assign_organization_role(&mut conn, student.id(), organization.id, "STUDENT").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;

    let join_request = request_course_join(&mut conn, student.id(), course.id)
        .await
        .expect("organization student should request linked course join");
    assert_eq!(join_request.status, COURSE_JOIN_STATUS_PENDING);

    let duplicate = request_course_join(&mut conn, student.id(), course.id)
        .await
        .expect("duplicate pending join request should be idempotent");
    assert_eq!(duplicate.id, join_request.id);

    let approved = decide_course_join_request(
        &mut conn,
        teacher.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_APPROVED.to_string(),
            decision_reason: Some("approved by teacher".to_string()),
        },
    )
    .await
    .expect("course teacher should approve join request");
    assert_eq!(approved.status, COURSE_JOIN_STATUS_APPROVED);
    assert_eq!(approved.reviewer_user_id, Some(teacher.id()));

    let enrolled = user_permission_course_request(
        &mut conn,
        student.id(),
        course.id,
        &Permissions::VIEW_COURSE.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(enrolled, "approved student should receive course bundle");
}

#[actix_web::test]
async fn platform_user_can_request_and_organization_admin_can_approve_join() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("JoinApprovalOrg")).await;
    let course = create_course(&mut conn, &unique_string("JoinApprovalCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;

    let requester = create_user_helper(&mut conn, "join_platform_user").await;
    let org_admin = create_user_helper(&mut conn, "join_org_admin").await;
    force_assign_platform_role(&mut conn, requester.id(), "USER").await;
    force_assign_organization_role(&mut conn, org_admin.id(), organization.id, "ADMIN").await;

    let join_request = request_course_join(&mut conn, requester.id(), course.id)
        .await
        .expect("platform user should request course join");

    let approved = decide_course_join_request(
        &mut conn,
        org_admin.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_APPROVED.to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect("organization admin should approve linked course join request");
    assert_eq!(approved.status, COURSE_JOIN_STATUS_APPROVED);
}

#[actix_web::test]
async fn user_without_scoped_join_permission_cannot_request() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "join_denied").await;
    let course = create_course(&mut conn, &unique_string("JoinDeniedCourse")).await;

    let denied = request_course_join(&mut conn, user.id(), course.id)
        .await
        .expect_err("user without join permission should be denied");
    assert!(matches!(denied, CourseEnrollmentError::PermissionDenied(_)));
}

#[actix_web::test]
async fn course_student_cannot_approve_join_request() {
    let mut conn = setup_conn().await;
    let requester = create_user_helper(&mut conn, "join_approval_requester").await;
    let reviewer = create_user_helper(&mut conn, "join_student_reviewer").await;
    let course = create_course(&mut conn, &unique_string("JoinReviewerDeniedCourse")).await;
    force_assign_platform_role(&mut conn, requester.id(), "USER").await;
    force_assign_course_role(&mut conn, reviewer.id(), course.id, "STUDENT").await;

    let join_request = request_course_join(&mut conn, requester.id(), course.id)
        .await
        .expect("requester should create join request");

    let denied = decide_course_join_request(
        &mut conn,
        reviewer.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_APPROVED.to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect_err("course student should not approve join request");
    assert!(matches!(denied, CourseEnrollmentError::PermissionDenied(_)));
}
