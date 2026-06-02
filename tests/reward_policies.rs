use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, organizations};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::models::reward_policy::{
    REWARD_PAYMENT_MINT, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
    REWARD_POLICY_SCOPE_PLATFORM,
};
use rust_learn::models::user::User;
use rust_learn::repositories::platform_repository::assign_role_to_user;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_policy_service::{
    create_reward_policy, list_reward_policies, CreateRewardPolicyRequest,
    ListRewardPoliciesRequest, RewardPolicyError,
};
use std::str::FromStr;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
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

fn platform_policy_request(amount: &str) -> CreateRewardPolicyRequest {
    CreateRewardPolicyRequest {
        scope_type: REWARD_POLICY_SCOPE_PLATFORM.to_string(),
        organization_id: None,
        course_id: None,
        event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
        token_amount: BigDecimal::from_str(amount).expect("valid amount"),
        multiplier: Some(BigDecimal::from(1)),
        max_payout: Some(BigDecimal::from(100)),
        cooldown_seconds: Some(86_400),
        payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
        active: Some(true),
    }
}

#[actix_web::test]
async fn platform_admin_creates_versioned_active_reward_policies() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "reward_policy_admin").await;
    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    let first = create_reward_policy(&mut conn, admin.id(), platform_policy_request("10"))
        .await
        .expect("admin should create first reward policy");
    assert_eq!(first.version, 1);
    assert!(first.active);

    let second = create_reward_policy(&mut conn, admin.id(), platform_policy_request("15"))
        .await
        .expect("admin should create second reward policy version");
    assert_eq!(second.version, 2);
    assert!(second.active);

    let active = list_reward_policies(
        &mut conn,
        admin.id(),
        ListRewardPoliciesRequest {
            scope_type: Some(REWARD_POLICY_SCOPE_PLATFORM.to_string()),
            event_type: Some(REWARD_EVENT_COURSE_COMPLETION.to_string()),
            active: Some(true),
            ..Default::default()
        },
    )
    .await
    .expect("admin should list active policies");
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, second.id);

    let inactive = list_reward_policies(
        &mut conn,
        admin.id(),
        ListRewardPoliciesRequest {
            scope_type: Some(REWARD_POLICY_SCOPE_PLATFORM.to_string()),
            event_type: Some(REWARD_EVENT_COURSE_COMPLETION.to_string()),
            active: Some(false),
            ..Default::default()
        },
    )
    .await
    .expect("admin should list inactive policies");
    assert!(inactive.iter().any(|policy| policy.id == first.id));
}

#[actix_web::test]
async fn platform_moderator_cannot_set_reward_policy() {
    let mut conn = setup_conn().await;
    let moderator = create_user_helper(&mut conn, "reward_policy_moderator").await;
    assign_role_to_user(&mut conn, moderator.id(), Roles::MODERATOR)
        .await
        .expect("failed to assign MODERATOR role");

    let denied = create_reward_policy(&mut conn, moderator.id(), platform_policy_request("10"))
        .await
        .expect_err("moderator should not set reward policy");
    assert!(matches!(denied, RewardPolicyError::PermissionDenied(_)));
}

#[actix_web::test]
async fn course_policy_requires_course_scope_and_can_explicitly_allow_mint() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "reward_policy_course_admin").await;
    let course = create_course(&mut conn, &unique_string("RewardPolicyCourse")).await;
    let organization = create_organization(&mut conn, &unique_string("RewardPolicyOrg")).await;
    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    let policy = create_reward_policy(
        &mut conn,
        admin.id(),
        CreateRewardPolicyRequest {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: Some(organization.id),
            course_id: Some(course.id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            token_amount: BigDecimal::from(20),
            multiplier: Some(BigDecimal::from_str("1.25").expect("valid multiplier")),
            max_payout: Some(BigDecimal::from(50)),
            cooldown_seconds: Some(3_600),
            payment_strategy: REWARD_PAYMENT_MINT.to_string(),
            active: Some(true),
        },
    )
    .await
    .expect("admin should create course reward policy");

    assert_eq!(policy.scope_type, REWARD_POLICY_SCOPE_COURSE);
    assert_eq!(policy.course_id, Some(course.id));
    assert_eq!(policy.organization_id, Some(organization.id));
    assert_eq!(policy.payment_strategy, REWARD_PAYMENT_MINT);

    let invalid = create_reward_policy(
        &mut conn,
        admin.id(),
        CreateRewardPolicyRequest {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            token_amount: BigDecimal::from(20),
            multiplier: None,
            max_payout: None,
            cooldown_seconds: None,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: Some(true),
        },
    )
    .await
    .expect_err("course policy should require course_id");
    assert!(matches!(invalid, RewardPolicyError::InvalidInput(_)));
}
