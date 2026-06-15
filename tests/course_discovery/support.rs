pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::learning::discover_courses::CourseDiscoveryUseCase;
pub(crate) use rust_learn::application::learning::get_learner_course_detail::LearnerCourseDetailUseCase;
pub(crate) use rust_learn::application::learning::get_learner_course_learning::LearnerCourseLearningUseCase;
pub(crate) use rust_learn::application::learning::learner_progress::LearnerProgressUseCase;
pub(crate) use rust_learn::application::learning::list_learner_course_catalog::LearnerCourseCatalogListUseCase;
pub(crate) use rust_learn::application::organizations::list_organization_courses::OrganizationCourseListUseCase;
pub(crate) use rust_learn::domain::learning::course::status::COURSE_STATUS_PUBLISHED;
pub(crate) use rust_learn::domain::learning::enrollment::status::COURSE_JOIN_STATUS_PENDING;
pub(crate) use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::learning::course_discovery_use_case::PostgresCourseDiscoveryUseCase;
pub(crate) use rust_learn::infra::postgres::learning::learner_course_catalog_list_use_case::PostgresLearnerCourseCatalogListUseCase;
pub(crate) use rust_learn::infra::postgres::learning::learner_course_detail_use_case::PostgresLearnerCourseDetailUseCase;
pub(crate) use rust_learn::infra::postgres::learning::learner_course_learning_use_case::PostgresLearnerCourseLearningUseCase;
pub(crate) use rust_learn::infra::postgres::learning::learner_progress_use_case::PostgresLearnerProgressUseCase;
pub(crate) use rust_learn::infra::postgres::models::chapter::NewChapter;
pub(crate) use rust_learn::infra::postgres::models::content::NewContent;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::course_join_request::NewCourseJoinRequest;
pub(crate) use rust_learn::infra::postgres::models::courses_organizations::NewCourseOrganization;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::reward_policy::NewRewardPolicy;
pub(crate) use rust_learn::infra::postgres::models::upload_job::NewUploadJob;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::organizations::organization_course_list_use_case::PostgresOrganizationCourseListUseCase;
pub(crate) use rust_learn::infra::postgres::schema::{
    chapters, contents, course_join_requests, courses, courses_organizations, organizations,
    reward_policies, upload_jobs,
};
pub(crate) use rust_learn::infra::postgres::{establish_connection, DbPool};
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use serde_json::Value;
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
pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection) -> User {
    let email = format!("{}@example.com", unique_string("course_discovery"));
    create_user(
        conn,
        "Course Discovery",
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
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
pub(crate) async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name)
        .await
        .expect("organization role should exist");
    organization_role_records::assign_organization_role_to_user(
        conn,
        user_id,
        organization_id,
        role_id,
    )
    .await
    .expect("failed to assign organization role");
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
pub(crate) fn course_discovery_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn CourseDiscoveryUseCase>> {
    web::Data::new(Arc::new(PostgresCourseDiscoveryUseCase::new(pool.clone())))
}
pub(crate) fn learner_progress_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn LearnerProgressUseCase>> {
    web::Data::new(Arc::new(PostgresLearnerProgressUseCase::new(pool.clone())))
}
pub(crate) fn learner_course_learning_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn LearnerCourseLearningUseCase>> {
    web::Data::new(Arc::new(PostgresLearnerCourseLearningUseCase::new(
        pool.clone(),
    )))
}
pub(crate) fn learner_course_detail_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn LearnerCourseDetailUseCase>> {
    web::Data::new(Arc::new(PostgresLearnerCourseDetailUseCase::new(
        pool.clone(),
    )))
}
pub(crate) fn learner_course_catalog_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn LearnerCourseCatalogListUseCase>> {
    web::Data::new(Arc::new(PostgresLearnerCourseCatalogListUseCase::new(
        pool.clone(),
    )))
}
pub(crate) fn organization_course_list_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationCourseListUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationCourseListUseCase::new(
        pool.clone(),
    )))
}
pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}
