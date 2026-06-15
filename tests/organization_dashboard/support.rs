pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::organizations::get_organization_dashboard::OrganizationDashboardUseCase;
pub(crate) use rust_learn::domain::learning::course::status::{
    COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED,
};
pub(crate) use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
pub(crate) use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
pub(crate) use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_FAILED,
};
pub(crate) use rust_learn::domain::teacher_applications::scope::TEACHER_APPLICATION_SCOPE_PLATFORM;
pub(crate) use rust_learn::domain::teacher_applications::status::TEACHER_APPLICATION_STATUS_SUBMITTED;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::courses_organizations::NewCourseOrganization;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::reward_candidate::NewRewardCandidate;
pub(crate) use rust_learn::infra::postgres::models::teacher_application::NewTeacherApplication;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::models::wallet::NewWallet;
pub(crate) use rust_learn::infra::postgres::organizations::organization_dashboard_use_case::PostgresOrganizationDashboardUseCase;
pub(crate) use rust_learn::infra::postgres::schema::{
    courses, courses_organizations, organizations, reward_candidates, teacher_applications, wallets,
};
pub(crate) use rust_learn::infra::postgres::{establish_connection, DbPool};
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use serde_json::{json, Value};
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

pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let email = format!("{}@example.com", unique_string(prefix));
    create_user(
        conn,
        &format!("{} User", prefix.replace('_', " ")),
        &email,
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
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

pub(crate) async fn create_course(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    title: &str,
    lifecycle_status: &str,
) -> Course {
    let course = diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result::<Course>(conn)
        .await
        .expect("failed to create course");

    diesel::update(courses::table.find(course.id))
        .set(courses::lifecycle_status.eq(lifecycle_status))
        .execute(conn)
        .await
        .expect("failed to set course lifecycle status");

    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id: course.id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course to organization");

    courses::table
        .find(course.id)
        .get_result(conn)
        .await
        .expect("failed to reload course")
}

pub(crate) fn organization_dashboard_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationDashboardUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationDashboardUseCase::new(
        pool.clone(),
    )))
}
