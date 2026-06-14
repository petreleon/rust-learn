use actix_service::Service;
use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::schema::{
    courses, courses_organizations, delegated_permissions, external_transactions,
    internal_transactions, organizations, reward_candidates, reward_execution_jobs,
    reward_fraud_blocks, reward_payout_records, reward_policies, reward_wallet_credit_records,
    teacher_applications, transactions, wallets,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::application::reporting::organization_reward_dashboard::OrganizationRewardDashboardUseCase;
use rust_learn::application::reporting::organization_summary::OrganizationSummaryUseCase;
use rust_learn::application::reporting::platform_fraud_dashboard::PlatformFraudDashboardUseCase;
use rust_learn::application::reporting::platform_reward_dashboard::PlatformRewardDashboardUseCase;
use rust_learn::application::reporting::platform_summary::PlatformSummaryUseCase;
use rust_learn::domain::access_control::delegation::DELEGATED_SCOPE_PLATFORM;
use rust_learn::domain::rewards::execution::RewardExecutionJobStatus;
use rust_learn::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TOKEN_CONFIRMED,
};
use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::domain::teacher_applications::scope::TEACHER_APPLICATION_SCOPE_PLATFORM;
use rust_learn::domain::teacher_applications::status::TEACHER_APPLICATION_STATUS_SUBMITTED;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::delegated_permission::NewDelegatedPermission;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::models::reward_fraud_block::NewRewardFraudBlock;
use rust_learn::models::reward_policy::NewRewardPolicy;
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::models::teacher_application::NewTeacherApplication;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::organization_role_records;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::models::wallet::NewWallet;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::infra::postgres::reporting::organization_reward_dashboard_use_case::PostgresOrganizationRewardDashboardUseCase;
use rust_learn::infra::postgres::reporting::organization_summary_use_case::PostgresOrganizationSummaryUseCase;
use rust_learn::infra::postgres::reporting::platform_fraud_dashboard_use_case::PostgresPlatformFraudDashboardUseCase;
use rust_learn::infra::postgres::reporting::platform_reward_dashboard_use_case::PostgresPlatformRewardDashboardUseCase;
use rust_learn::infra::postgres::reporting::platform_summary_use_case::PostgresPlatformSummaryUseCase;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

fn platform_summary_use_case(pool: &DbPool) -> web::Data<Arc<dyn PlatformSummaryUseCase>> {
    web::Data::new(
        Arc::new(PostgresPlatformSummaryUseCase::new(pool.clone()))
            as Arc<dyn PlatformSummaryUseCase>,
    )
}

fn organization_summary_use_case(pool: &DbPool) -> web::Data<Arc<dyn OrganizationSummaryUseCase>> {
    web::Data::new(
        Arc::new(PostgresOrganizationSummaryUseCase::new(pool.clone()))
            as Arc<dyn OrganizationSummaryUseCase>,
    )
}

fn organization_reward_dashboard_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationRewardDashboardUseCase>> {
    web::Data::new(
        Arc::new(PostgresOrganizationRewardDashboardUseCase::new(pool.clone()))
            as Arc<dyn OrganizationRewardDashboardUseCase>,
    )
}

fn platform_fraud_dashboard_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn PlatformFraudDashboardUseCase>> {
    web::Data::new(
        Arc::new(PostgresPlatformFraudDashboardUseCase::new(pool.clone()))
            as Arc<dyn PlatformFraudDashboardUseCase>,
    )
}

fn platform_reward_dashboard_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn PlatformRewardDashboardUseCase>> {
    web::Data::new(
        Arc::new(PostgresPlatformRewardDashboardUseCase::new(pool.clone()))
            as Arc<dyn PlatformRewardDashboardUseCase>,
    )
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
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("platform role should exist");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name)
        .await
        .expect("organization role should exist");
    organization_role_records::assign_organization_role_to_user(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

async fn create_organization(conn: &mut AsyncPgConnection) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: unique_string("ReportingOrg"),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

async fn create_course(conn: &mut AsyncPgConnection) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string("ReportingCourse"),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}
