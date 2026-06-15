use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::sync::Arc;
use rust_learn::application::wallet::audit_wallet::WalletAuditUseCase;
use rust_learn::application::wallet::create_deposit_intent::{
    create_deposit_intent, WalletDepositIntentRequest as WalletTokenTransferRequest,
    WalletDepositIntentUseCase, WalletDepositIntentView,
};
use rust_learn::application::wallet::index_deposit::{
    index_observed_deposit, ObservedWalletDepositEvent, WalletDepositIndexOutput,
};
use rust_learn::application::wallet::link_wallet::{
    link_wallet, LinkedWalletView, WalletLinkSubject, WalletLinkUseCase,
};
use rust_learn::application::wallet::manage_token_tax::WalletTokenTaxUseCase;
use rust_learn::application::wallet::read_wallet::WalletReadUseCase;
use rust_learn::application::wallet::retire_tokens::WalletRetirementUseCase;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::schema::{
    courses, external_transactions, internal_transactions, organization_roles, organizations,
    platform_roles, reward_candidates, reward_payout_records, reward_wallet_credit_records,
    role_permission_organization, role_permission_platform, transactions,
    transactions_external_transactions, transactions_internal_transactions,
    users, wallet_token_deposit_intents, wallets,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_WALLET_CREDITED,
};
use rust_learn::models::course::NewCourse;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::organization_role_records;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::infra::postgres::operations::persistent_state::set_persistent_state;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
use rust_learn::infra::postgres::wallet::wallet_audit_use_case::PostgresWalletAuditUseCase;
use rust_learn::infra::postgres::wallet::wallet_deposit_index_store::PostgresWalletDepositIndexStore;
use rust_learn::infra::postgres::wallet::wallet_deposit_intent_store::PostgresWalletDepositIntentStore;
use rust_learn::infra::postgres::wallet::wallet_deposit_intent_use_case::PostgresWalletDepositIntentUseCase;
use rust_learn::infra::postgres::wallet::wallet_link_store::PostgresWalletLinkStore;
use rust_learn::infra::postgres::wallet::wallet_link_use_case::PostgresWalletLinkUseCase;
use rust_learn::infra::postgres::wallet::wallet_read_use_case::PostgresWalletReadUseCase;
use rust_learn::infra::postgres::wallet::wallet_retirement_use_case::PostgresWalletRetirementUseCase;
use rust_learn::infra::postgres::wallet::wallet_token_tax_use_case::PostgresWalletTokenTaxUseCase;
use rust_learn::infra::tokens::jwt::create_jwt;
use serde_json::json;
use serde_json::Value;

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

async fn mark_user_kyc_verified(conn: &mut AsyncPgConnection, user_id: i32) {
    diesel::update(users::table.find(user_id))
        .set(users::kyc_verified.eq(true))
        .execute(conn)
        .await
        .expect("failed to mark user KYC verified");
}

async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<LinkedWalletView, rust_learn::application::wallet::link_wallet::WalletLinkError> {
    let mut store = PostgresWalletLinkStore::new(conn);
    link_wallet(&mut store, user_id, WalletLinkSubject::OwnUser).await
}

async fn link_organization_wallet(
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

async fn deposit_tokens_to_user_wallet(
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

async fn credit_observed_wallet_deposit(
    conn: &mut AsyncPgConnection,
    event: ObservedWalletDepositEvent,
) -> Result<
    WalletDepositIndexOutput,
    rust_learn::application::wallet::index_deposit::WalletDepositIndexError,
> {
    let mut store = PostgresWalletDepositIndexStore::new(conn);
    index_observed_deposit(&mut store, event).await
}

async fn create_test_organization(conn: &mut AsyncPgConnection) -> Organization {
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

async fn create_test_course(conn: &mut AsyncPgConnection) -> i32 {
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

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("platform role should exist");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}
