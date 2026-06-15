pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel_async::AsyncPgConnection;
pub(crate) use rust_learn::application::access_control::manage_delegated_permissions::DelegatedPermissionUseCase;
pub(crate) use rust_learn::application::rewards::manage_fraud_block::RewardFraudBlockUseCase;
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::{establish_connection, DbPool};
pub(crate) use rust_learn::infra::postgres::access_control::delegated_permissions::use_case::PostgresDelegatedPermissionUseCase;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::rewards::reward_fraud_block_use_case::PostgresRewardFraudBlockUseCase;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use rust_learn::models::user::User;
pub(crate) use serde_json::{json, Value};
pub(crate) use std::sync::Arc;

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
    let email = format!("{}@example.com", unique_string(name));
    create_user(
        conn,
        name,
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

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) fn reward_fraud_block_use_case(pool: &DbPool) -> Arc<dyn RewardFraudBlockUseCase> {
    Arc::new(PostgresRewardFraudBlockUseCase::new(pool.clone()))
}

pub(crate) fn delegated_permission_use_case(pool: &DbPool) -> Arc<dyn DelegatedPermissionUseCase> {
    Arc::new(PostgresDelegatedPermissionUseCase::new(pool.clone()))
}
