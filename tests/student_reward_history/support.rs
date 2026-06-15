pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::rewards::list_reward_history::StudentRewardHistoryUseCase;
pub(crate) use rust_learn::application::wallet::link_wallet::{
    link_wallet, LinkedWalletView, WalletLinkSubject,
};
pub(crate) use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
pub(crate) use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
pub(crate) use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_WALLET_CREDITED,
};
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::establish_connection;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::reward_candidate::NewRewardCandidate;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::rewards::reward_history_use_case::PostgresStudentRewardHistoryUseCase;
pub(crate) use rust_learn::infra::postgres::schema::{
    courses, external_transactions, internal_transactions, reward_candidates,
    reward_payout_records, reward_wallet_credit_records, transactions,
    transactions_external_transactions, transactions_internal_transactions, users,
};
pub(crate) use rust_learn::infra::postgres::wallet::wallet_link_store::PostgresWalletLinkStore;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use serde_json::{json, Value};
pub(crate) use std::sync::Arc;

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) fn student_reward_history_use_case(
    pool: &rust_learn::infra::postgres::DbPool,
) -> Arc<dyn StudentRewardHistoryUseCase> {
    Arc::new(PostgresStudentRewardHistoryUseCase::new(pool.clone()))
}

pub(crate) async fn setup_conn(
    pool: &rust_learn::infra::postgres::DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_verified_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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

pub(crate) async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<LinkedWalletView, rust_learn::application::wallet::link_wallet::WalletLinkError> {
    let mut store = PostgresWalletLinkStore::new(conn);
    link_wallet(&mut store, user_id, WalletLinkSubject::OwnUser).await
}

pub(crate) async fn create_course(conn: &mut AsyncPgConnection, title_prefix: &str) -> Course {
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
