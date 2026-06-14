use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::sync::Arc;
use rust_learn::application::wallet::audit_wallet::WalletAuditUseCase;
use rust_learn::application::wallet::create_deposit_intent::WalletDepositIntentUseCase;
use rust_learn::application::wallet::link_wallet::WalletLinkUseCase;
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
use rust_learn::models::role::{OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::repositories::persistent_state_repository::set_persistent_state;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::infra::postgres::wallet::wallet_audit_use_case::PostgresWalletAuditUseCase;
use rust_learn::infra::postgres::wallet::wallet_deposit_intent_use_case::PostgresWalletDepositIntentUseCase;
use rust_learn::infra::postgres::wallet::wallet_link_use_case::PostgresWalletLinkUseCase;
use rust_learn::infra::postgres::wallet::wallet_read_use_case::PostgresWalletReadUseCase;
use rust_learn::infra::postgres::wallet::wallet_retirement_use_case::PostgresWalletRetirementUseCase;
use rust_learn::infra::postgres::wallet::wallet_token_tax_use_case::PostgresWalletTokenTaxUseCase;
use rust_learn::services::wallet_service::{
    self, credit_observed_wallet_deposit, ObservedWalletDepositEvent, WalletTokenTransferRequest,
};
use rust_learn::utils::jwt_utils::create_jwt;
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
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}
