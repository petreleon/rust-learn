use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{
    chapters, contents, course_join_requests, course_progress, courses, courses_organizations,
    organizations, reward_candidates, reward_policies,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_FAILED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TEACHER_APPROVED,
};
use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::chapter::NewChapter;
use rust_learn::models::content::NewContent;
use rust_learn::models::course::{Course, NewCourse, COURSE_STATUS_PUBLISHED};
use rust_learn::models::course_join_request::{
    NewCourseJoinRequest, COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_PENDING,
    COURSE_JOIN_STATUS_WAITLISTED,
};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::models::reward_policy::NewRewardPolicy;
use rust_learn::models::role::{CourseRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::application::learning::get_teacher_course_enrollment_workspace::TeacherCourseEnrollmentWorkspaceUseCase;
use rust_learn::application::learning::get_teacher_course_students::TeacherCourseStudentsUseCase;
use rust_learn::application::learning::get_teacher_course_workspace::TeacherCourseWorkspaceUseCase;
use rust_learn::application::learning::list_teacher_course_dashboard::TeacherCourseDashboardListUseCase;
use rust_learn::infra::postgres::learning::teacher_course_dashboard_list_use_case::PostgresTeacherCourseDashboardListUseCase;
use rust_learn::infra::postgres::learning::teacher_course_enrollment_workspace_use_case::PostgresTeacherCourseEnrollmentWorkspaceUseCase;
use rust_learn::infra::postgres::learning::teacher_course_students_use_case::PostgresTeacherCourseStudentsUseCase;
use rust_learn::infra::postgres::learning::teacher_course_workspace_use_case::PostgresTeacherCourseWorkspaceUseCase;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::{json, Value};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, counter)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

fn teacher_dashboard_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn TeacherCourseDashboardListUseCase>> {
    web::Data::new(Arc::new(PostgresTeacherCourseDashboardListUseCase::new(
        pool.clone(),
    )))
}

fn teacher_workspace_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn TeacherCourseWorkspaceUseCase>> {
    web::Data::new(Arc::new(PostgresTeacherCourseWorkspaceUseCase::new(
        pool.clone(),
    )))
}

fn teacher_students_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn TeacherCourseStudentsUseCase>> {
    web::Data::new(Arc::new(PostgresTeacherCourseStudentsUseCase::new(
        pool.clone(),
    )))
}

fn teacher_enrollment_workspace_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn TeacherCourseEnrollmentWorkspaceUseCase>> {
    web::Data::new(Arc::new(
        PostgresTeacherCourseEnrollmentWorkspaceUseCase::new(pool.clone()),
    ))
}

async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    create_user(
        conn,
        &format!("{} User", prefix),
        &format!("{}@example.com", unique_string(prefix)),
        Some(NaiveDate::from_ymd_opt(1995, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role should exist");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn publish_course(conn: &mut AsyncPgConnection, course_id: i32) {
    diesel::update(courses::table.find(course_id))
        .set(courses::lifecycle_status.eq(COURSE_STATUS_PUBLISHED))
        .execute(conn)
        .await
        .expect("failed to publish course");
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
