pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::learning::get_teacher_course_enrollment_workspace::TeacherCourseEnrollmentWorkspaceUseCase;
pub(crate) use rust_learn::application::learning::get_teacher_course_students::TeacherCourseStudentsUseCase;
pub(crate) use rust_learn::application::learning::get_teacher_course_workspace::TeacherCourseWorkspaceUseCase;
pub(crate) use rust_learn::application::learning::list_teacher_course_dashboard::TeacherCourseDashboardListUseCase;
pub(crate) use rust_learn::db::schema::{
    chapters, contents, course_join_requests, course_progress, courses, courses_organizations,
    organizations, reward_candidates, reward_policies,
};
pub(crate) use rust_learn::db::{DbPool, establish_connection};
pub(crate) use rust_learn::domain::learning::course::status::COURSE_STATUS_PUBLISHED;
pub(crate) use rust_learn::domain::learning::enrollment::status::{
    COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED,
};
pub(crate) use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
pub(crate) use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_FAILED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TEACHER_APPROVED,
};
pub(crate) use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::learning::teacher_course_dashboard_list_use_case::PostgresTeacherCourseDashboardListUseCase;
pub(crate) use rust_learn::infra::postgres::learning::teacher_course_enrollment_workspace_use_case::PostgresTeacherCourseEnrollmentWorkspaceUseCase;
pub(crate) use rust_learn::infra::postgres::learning::teacher_course_students_use_case::PostgresTeacherCourseStudentsUseCase;
pub(crate) use rust_learn::infra::postgres::learning::teacher_course_workspace_use_case::PostgresTeacherCourseWorkspaceUseCase;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use rust_learn::infra::postgres::models::chapter::NewChapter;
pub(crate) use rust_learn::infra::postgres::models::content::NewContent;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::course_join_request::NewCourseJoinRequest;
pub(crate) use rust_learn::infra::postgres::models::courses_organizations::NewCourseOrganization;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::reward_candidate::NewRewardCandidate;
pub(crate) use rust_learn::infra::postgres::models::reward_policy::NewRewardPolicy;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use serde_json::{Value, json};
pub(crate) use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, counter)
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) fn teacher_dashboard_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn TeacherCourseDashboardListUseCase>> {
    web::Data::new(Arc::new(PostgresTeacherCourseDashboardListUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn teacher_workspace_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn TeacherCourseWorkspaceUseCase>> {
    web::Data::new(Arc::new(PostgresTeacherCourseWorkspaceUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn teacher_students_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn TeacherCourseStudentsUseCase>> {
    web::Data::new(Arc::new(PostgresTeacherCourseStudentsUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn teacher_enrollment_workspace_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn TeacherCourseEnrollmentWorkspaceUseCase>> {
    web::Data::new(Arc::new(
        PostgresTeacherCourseEnrollmentWorkspaceUseCase::new(pool.clone()),
    ))
}

pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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

pub(crate) async fn assign_platform_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("platform role should exist");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

pub(crate) async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("course role should exist");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

pub(crate) async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
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

pub(crate) async fn publish_course(conn: &mut AsyncPgConnection, course_id: i32) {
    diesel::update(courses::table.find(course_id))
        .set(courses::lifecycle_status.eq(COURSE_STATUS_PUBLISHED))
        .execute(conn)
        .await
        .expect("failed to publish course");
}

pub(crate) async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
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
