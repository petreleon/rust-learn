use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, RewardFraudBlockError, RewardFraudBlockUseCase,
};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::schema::{courses, notifications, organizations, reward_fraud_blocks};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use rust_learn::infra::postgres::rewards::reward_fraud_block_use_case::PostgresRewardFraudBlockUseCase;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::delegated_permission::{
    GrantDelegatedPermissionRequest, DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM,
};
use rust_learn::models::notification::Notification;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_fraud_block::RewardFraudBlock;
use rust_learn::models::role::{OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::delegated_permission_service::grant_delegated_permission;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

fn setup_pool() -> DbPool {
    let _ = dotenvy::dotenv();
    establish_connection()
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
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

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
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

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role not found");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn force_assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role not found");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

fn reward_fraud_block_use_case(pool: &DbPool) -> PostgresRewardFraudBlockUseCase {
    PostgresRewardFraudBlockUseCase::new(pool.clone())
}

async fn count_fraud_block_notifications(
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

fn teacher_block_request(teacher_user_id: i32) -> CreateRewardFraudBlockCommand {
    CreateRewardFraudBlockCommand {
        scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
        teacher_user_id: Some(teacher_user_id),
        organization_id: None,
        course_id: None,
        reward_policy_id: None,
        reason: "suspicious reward approvals".to_string(),
        evidence_reference: Some("case://teacher-block".to_string()),
        expires_at: None,
    }
}
