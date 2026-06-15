pub(crate) use actix_web::{http::StatusCode, test};
pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::{NaiveDate, Utc};
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::wallet::create_deposit_intent::{
    create_deposit_intent, WalletDepositIntentRequest as WalletTokenTransferRequest,
    WalletDepositIntentView,
};
pub(crate) use rust_learn::application::wallet::index_deposit::{
    index_observed_deposit, ObservedWalletDepositEvent, WalletDepositIndexOutput,
};
pub(crate) use rust_learn::application::wallet::link_wallet::{
    link_wallet, LinkedWalletView, WalletLinkSubject,
};
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::schema::{
    courses, external_transactions, internal_transactions, organization_roles, organizations,
    platform_roles, reward_candidates, reward_payout_records, reward_wallet_credit_records,
    role_permission_organization, role_permission_platform, transactions,
    transactions_external_transactions, transactions_internal_transactions, users,
    wallet_token_deposit_intents, wallets,
};
pub(crate) use rust_learn::db::{establish_connection, DbPool};
pub(crate) use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
pub(crate) use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
pub(crate) use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_WALLET_CREDITED,
};
pub(crate) use rust_learn::domain::wallet::deposit::{WalletDepositEventType, WalletDepositStatus};
use rust_learn::infra::postgres::access_control::{platform_role_records, role_catalog_store};
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::operations::persistent_state::set_persistent_state;
use rust_learn::infra::postgres::wallet::wallet_deposit_index_store::PostgresWalletDepositIndexStore;
use rust_learn::infra::postgres::wallet::wallet_deposit_intent_store::PostgresWalletDepositIntentStore;
use rust_learn::infra::postgres::wallet::wallet_link_store::PostgresWalletLinkStore;
pub(crate) use rust_learn::models::course::NewCourse;
pub(crate) use rust_learn::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::models::user::User;
pub(crate) use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let seq = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, seq)
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

pub(crate) async fn mark_user_kyc_verified(conn: &mut AsyncPgConnection, user_id: i32) {
    diesel::update(users::table.find(user_id))
        .set(users::kyc_verified.eq(true))
        .execute(conn)
        .await
        .expect("failed to mark user KYC verified");
}

pub(crate) async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<LinkedWalletView, rust_learn::application::wallet::link_wallet::WalletLinkError> {
    let mut store = PostgresWalletLinkStore::new(conn);
    link_wallet(&mut store, user_id, WalletLinkSubject::OwnUser).await
}

pub(crate) async fn link_organization_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<LinkedWalletView, rust_learn::application::wallet::link_wallet::WalletLinkError> {
    let mut store = PostgresWalletLinkStore::new(conn);
    link_wallet(
        &mut store,
        actor_user_id,
        WalletLinkSubject::Organization(organization_id),
    )
    .await
}

pub(crate) async fn deposit_tokens_to_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    request: WalletTokenTransferRequest,
) -> Result<
    WalletDepositIntentView,
    rust_learn::application::wallet::create_deposit_intent::WalletDepositIntentError,
> {
    let mut store = PostgresWalletDepositIntentStore::new(conn);
    create_deposit_intent(&mut store, user_id, request).await
}

pub(crate) async fn credit_observed_wallet_deposit(
    conn: &mut AsyncPgConnection,
    event: ObservedWalletDepositEvent,
) -> Result<
    WalletDepositIndexOutput,
    rust_learn::application::wallet::index_deposit::WalletDepositIndexError,
> {
    let mut store = PostgresWalletDepositIndexStore::new(conn);
    index_observed_deposit(&mut store, event).await
}

pub(crate) async fn create_test_organization(conn: &mut AsyncPgConnection) -> Organization {
    let new_org = NewOrganization {
        name: unique_string("wallet_org"),
        website_link: None,
        profile_url: None,
    };

    diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

pub(crate) async fn create_test_course(conn: &mut AsyncPgConnection) -> i32 {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string("wallet_audit_course"),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .returning(courses::id)
        .get_result(conn)
        .await
        .expect("failed to create wallet audit course")
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
