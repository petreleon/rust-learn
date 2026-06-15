pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::{NaiveDate, Utc};
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::rewards::record_token_confirmation::RewardTokenConfirmationCommand as RewardTokenConfirmationRequest;
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::establish_connection;
pub(crate) use rust_learn::db::schema::{
    courses, external_transactions, internal_transactions, notifications, platform_roles,
    reward_candidates, reward_payout_records, reward_policies, reward_wallet_credit_records,
    role_permission_platform, transactions, transactions_external_transactions,
    transactions_internal_transactions, wallets,
};
pub(crate) use rust_learn::domain::rewards::audit::{
    REWARD_AUDIT_EVENT_TOKEN_CONFIRMED, REWARD_AUDIT_EVENT_WALLET_CREDITED,
    REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED,
};
pub(crate) use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
pub(crate) use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
pub(crate) use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_NOTIFIED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING, REWARD_STATUS_WALLET_CREDITED,
};
pub(crate) use rust_learn::domain::rewards::payout::{
    REWARD_PAYOUT_METHOD_MINT, REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER,
};
pub(crate) use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN, REWARD_PAYMENT_TREASURY_TRANSFER,
    REWARD_POLICY_SCOPE_COURSE,
};
pub(crate) use rust_learn::domain::rewards::wallet_credit::REWARD_TRANSACTION_TYPE_WALLET_CREDIT;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::operations::persistent_state::set_persistent_state;
pub(crate) use rust_learn::infra::postgres::rewards::reward_audit_records::list_reward_audit_events;
pub(crate) use rust_learn::models::course::{Course, NewCourse};
pub(crate) use rust_learn::models::reward_candidate::{NewRewardCandidate, RewardCandidate};
pub(crate) use rust_learn::models::reward_policy::NewRewardPolicy;
pub(crate) use rust_learn::models::user::User;
pub(crate) use serde_json::json;

use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, PartialEq)]
pub(crate) enum RewardExecutionError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidInput(String),
    NoActivePolicy,
    Database(String),
}

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let seq = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, seq)
}

pub(crate) fn unique_hash(prefix: &str) -> String {
    let seq = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!(
        "0x{}{:x}{:x}{:x}",
        prefix,
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
        seq
    )
}

pub(crate) async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user")
}

pub(crate) async fn create_custom_platform_role(
    conn: &mut AsyncPgConnection,
    role_name: &str,
    permissions: &[Permissions],
) -> i32 {
    let role_id = diesel::insert_into(platform_roles::table)
        .values((
            platform_roles::name.eq(role_name),
            platform_roles::description.eq(Some(
                "test-only reward execution permission bundle".to_string(),
            )),
        ))
        .returning(platform_roles::id)
        .get_result::<i32>(conn)
        .await
        .expect("failed to create custom platform role");

    for permission in permissions {
        diesel::insert_into(role_permission_platform::table)
            .values((
                role_permission_platform::platform_role_id.eq(Some(role_id)),
                role_permission_platform::permission.eq(permission.to_string()),
            ))
            .execute(conn)
            .await
            .expect("failed to assign custom platform role permission");
    }

    role_id
}

pub(crate) async fn assign_platform_role_id(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    role_id: i32,
) {
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign custom platform role");
}
