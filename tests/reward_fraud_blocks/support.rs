pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionError, DelegatedPermissionOutput, DelegatedPermissionUseCase,
    GrantDelegatedPermissionCommand,
};
pub(crate) use rust_learn::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, ListRewardFraudBlocksQuery, RewardFraudBlockError,
    RewardFraudBlockUseCase,
};
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::schema::{
    courses, notifications, organizations, reward_fraud_blocks,
};
pub(crate) use rust_learn::db::{establish_connection, DbPool};
pub(crate) use rust_learn::domain::access_control::delegation::{
    DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM,
};
pub(crate) use rust_learn::domain::rewards::fraud_block::{
    RewardFraudBlockScope, REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
pub(crate) use rust_learn::infra::postgres::access_control::delegated_permissions::use_case::PostgresDelegatedPermissionUseCase;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::notification::Notification;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::reward_fraud_block::RewardFraudBlock;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::rewards::reward_fraud_block_use_case::PostgresRewardFraudBlockUseCase;

pub(crate) struct GrantDelegatedPermissionRequest {
    pub(crate) grantee_user_id: i32,
    pub(crate) permission: String,
    pub(crate) scope_type: String,
    pub(crate) organization_id: Option<i32>,
    pub(crate) course_id: Option<i32>,
    pub(crate) reason: Option<String>,
    pub(crate) expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

pub(crate) fn setup_pool() -> DbPool {
    let _ = dotenvy::dotenv();
    establish_connection()
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn grant_delegated_permission(
    _conn: &mut AsyncPgConnection,
    grantor_user_id: i32,
    request: GrantDelegatedPermissionRequest,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
    let pool = establish_connection();
    PostgresDelegatedPermissionUseCase::new(pool)
        .grant_delegated_permission(GrantDelegatedPermissionCommand {
            course_id: request.course_id,
            expires_at: request.expires_at,
            grantee_user_id: request.grantee_user_id,
            grantor_user_id,
            organization_id: request.organization_id,
            permission: request.permission,
            reason: request.reason,
            scope_type: request.scope_type,
        })
        .await
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

pub(crate) async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

pub(crate) async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: name.to_string(),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

pub(crate) async fn force_assign_platform_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("platform role not found");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

pub(crate) async fn force_assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name)
        .await
        .expect("organization role not found");
    organization_role_records::assign_organization_role_to_user(
        conn,
        user_id,
        organization_id,
        role_id,
    )
    .await
    .expect("failed to assign organization role");
}

pub(crate) fn reward_fraud_block_use_case(pool: &DbPool) -> PostgresRewardFraudBlockUseCase {
    PostgresRewardFraudBlockUseCase::new(pool.clone())
}

pub(crate) async fn count_fraud_block_notifications(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    title: &str,
) -> i64 {
    notifications::table
        .filter(notifications::user_id.eq(Some(user_id)))
        .filter(notifications::title.eq(title))
        .count()
        .get_result::<i64>(conn)
        .await
        .expect("fraud block notifications should be countable")
}
