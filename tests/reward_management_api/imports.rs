use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel_async::AsyncPgConnection;
use rust_learn::application::rewards::manage_fraud_block::RewardFraudBlockUseCase;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::infra::postgres::rewards::reward_fraud_block_use_case::PostgresRewardFraudBlockUseCase;
use rust_learn::models::role::PlatformRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::{json, Value};
use std::sync::Arc;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
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

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn reward_fraud_block_use_case(pool: &DbPool) -> Arc<dyn RewardFraudBlockUseCase> {
    Arc::new(PostgresRewardFraudBlockUseCase::new(pool.clone()))
}
