use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::organizations::get_organization_dashboard::OrganizationDashboardUseCase;
use rust_learn::db::schema::{
    courses, courses_organizations, organizations, reward_candidates, teacher_applications, wallets,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::learning::course::status::{
    COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED,
};
use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_FAILED,
};
use rust_learn::domain::teacher_applications::scope::TEACHER_APPLICATION_SCOPE_PLATFORM;
use rust_learn::domain::teacher_applications::status::TEACHER_APPLICATION_STATUS_SUBMITTED;
use rust_learn::infra::postgres::organizations::organization_dashboard_use_case::PostgresOrganizationDashboardUseCase;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::models::role::OrganizationRole;
use rust_learn::models::teacher_application::NewTeacherApplication;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::organization_role_records;
use rust_learn::models::wallet::NewWallet;
use rust_learn::repositories::user_repository::create_user;
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

async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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

async fn create_course(
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

fn organization_dashboard_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationDashboardUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationDashboardUseCase::new(
        pool.clone(),
    )))
}
