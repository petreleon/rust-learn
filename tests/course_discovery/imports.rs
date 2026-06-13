use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{
    chapters, contents, course_join_requests, courses, courses_organizations, organizations,
    reward_policies, upload_jobs,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::application::learning::discover_courses::CourseDiscoveryUseCase;
use rust_learn::application::learning::get_learner_course_detail::LearnerCourseDetailUseCase;
use rust_learn::application::learning::get_learner_course_learning::LearnerCourseLearningUseCase;
use rust_learn::application::learning::learner_progress::LearnerProgressUseCase;
use rust_learn::application::learning::list_learner_course_catalog::LearnerCourseCatalogListUseCase;
use rust_learn::infra::postgres::learning::course_discovery_use_case::PostgresCourseDiscoveryUseCase;
use rust_learn::infra::postgres::learning::learner_course_catalog_list_use_case::PostgresLearnerCourseCatalogListUseCase;
use rust_learn::infra::postgres::learning::learner_course_detail_use_case::PostgresLearnerCourseDetailUseCase;
use rust_learn::infra::postgres::learning::learner_course_learning_use_case::PostgresLearnerCourseLearningUseCase;
use rust_learn::infra::postgres::learning::learner_progress_use_case::PostgresLearnerProgressUseCase;
use rust_learn::models::chapter::NewChapter;
use rust_learn::models::content::NewContent;
use rust_learn::models::course::{Course, NewCourse, COURSE_STATUS_PUBLISHED};
use rust_learn::models::course_join_request::{NewCourseJoinRequest, COURSE_JOIN_STATUS_PENDING};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_policy::NewRewardPolicy;
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::upload_job::NewUploadJob;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::Value;
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

async fn create_test_user(conn: &mut AsyncPgConnection) -> User {
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

async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role should exist");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
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

fn course_discovery_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn CourseDiscoveryUseCase>> {
    web::Data::new(Arc::new(PostgresCourseDiscoveryUseCase::new(pool.clone())))
}

fn learner_progress_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn LearnerProgressUseCase>> {
    web::Data::new(Arc::new(PostgresLearnerProgressUseCase::new(pool.clone())))
}

fn learner_course_learning_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn LearnerCourseLearningUseCase>> {
    web::Data::new(Arc::new(PostgresLearnerCourseLearningUseCase::new(
        pool.clone(),
    )))
}

fn learner_course_detail_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn LearnerCourseDetailUseCase>> {
    web::Data::new(Arc::new(PostgresLearnerCourseDetailUseCase::new(
        pool.clone(),
    )))
}

fn learner_course_catalog_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn LearnerCourseCatalogListUseCase>> {
    web::Data::new(Arc::new(PostgresLearnerCourseCatalogListUseCase::new(
        pool.clone(),
    )))
}
