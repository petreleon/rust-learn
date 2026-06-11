use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{
    courses, external_transactions, internal_transactions, reward_candidates,
    reward_payout_records, reward_wallet_credit_records, transactions,
    transactions_external_transactions, transactions_internal_transactions, users,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_WALLET_CREDITED,
};
use rust_learn::models::role::CourseRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::wallet::Wallet;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::wallet_service;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::{json, Value};

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

async fn setup_conn(
    pool: &rust_learn::db::DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_verified_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let email = format!("{}@example.com", unique_string(prefix));
    let user = create_user(
        conn,
        prefix,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
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

async fn create_course(conn: &mut AsyncPgConnection, title_prefix: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string(title_prefix),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
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
