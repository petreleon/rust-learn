use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::wallet::link_wallet::{
    link_wallet, LinkedWalletView, WalletLinkSubject,
};
use rust_learn::application::rewards::list_reward_history::StudentRewardHistoryUseCase;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{
    courses, external_transactions, internal_transactions, reward_candidates,
    reward_payout_records, reward_wallet_credit_records, transactions,
    transactions_external_transactions, transactions_internal_transactions, users,
};
use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_WALLET_CREDITED,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::course_role_records;
use rust_learn::infra::postgres::rewards::reward_history_use_case::PostgresStudentRewardHistoryUseCase;
use rust_learn::infra::postgres::wallet::wallet_link_store::PostgresWalletLinkStore;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
use rust_learn::infra::tokens::jwt::create_jwt;
use serde_json::{json, Value};
use std::sync::Arc;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn student_reward_history_use_case(
    pool: &rust_learn::db::DbPool,
) -> Arc<dyn StudentRewardHistoryUseCase> {
    Arc::new(PostgresStudentRewardHistoryUseCase::new(pool.clone()))
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
        .set((users::email_verified.eq(true), users::kyc_verified.eq(true)))
        .execute(conn)
        .await
        .expect("failed to verify user");

    user
}

async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<LinkedWalletView, rust_learn::application::wallet::link_wallet::WalletLinkError> {
    let mut store = PostgresWalletLinkStore::new(conn);
    link_wallet(&mut store, user_id, WalletLinkSubject::OwnUser).await
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
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("course role should exist");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}
