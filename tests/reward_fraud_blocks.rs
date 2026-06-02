use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, notifications, organizations, reward_fraud_blocks};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::notification::Notification;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_fraud_block::{
    RewardFraudBlock, REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use rust_learn::models::role::{OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_fraud_block_service::{
    create_reward_fraud_block, revoke_reward_fraud_block, RewardFraudBlockError,
    RewardFraudBlockRequest,
};

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

fn teacher_block_request(teacher_user_id: i32) -> RewardFraudBlockRequest {
    RewardFraudBlockRequest {
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

#[actix_web::test]
async fn platform_permissions_create_and_revoke_reward_fraud_blocks() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "fraud_block_admin").await;
    let moderator = create_user_helper(&mut conn, "fraud_block_moderator").await;
    let teacher = create_user_helper(&mut conn, "fraud_block_teacher").await;
    let org_operator = create_user_helper(&mut conn, "fraud_block_org_operator").await;
    let organization = create_organization(&mut conn, &unique_string("FraudBlockOrg")).await;
    let course = create_course(&mut conn, &unique_string("FraudBlockCourse")).await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    force_assign_platform_role(&mut conn, moderator.id(), "MODERATOR").await;
    force_assign_organization_role(&mut conn, org_operator.id(), organization.id, "ADMIN").await;

    let denied = create_reward_fraud_block(
        &mut conn,
        moderator.id(),
        teacher_block_request(teacher.id()),
    )
    .await
    .expect_err("moderator should not get teacher fraud block permission by default");
    assert!(matches!(denied, RewardFraudBlockError::PermissionDenied(_)));

    let teacher_block =
        create_reward_fraud_block(&mut conn, admin.id(), teacher_block_request(teacher.id()))
            .await
            .expect("admin should block teacher reward activity");
    assert_eq!(teacher_block.scope_type, REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    assert_eq!(teacher_block.teacher_user_id, Some(teacher.id()));
    assert_eq!(teacher_block.created_by_user_id, admin.id());
    assert_eq!(teacher_block.reason, "suspicious reward approvals");
    assert_eq!(
        teacher_block.evidence_reference.as_deref(),
        Some("case://teacher-block")
    );
    assert!(teacher_block.revoked_at.is_none());
    let teacher_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(teacher.id())))
        .filter(notifications::title.eq("reward_fraud_block:created"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("teacher fraud block notifications should be countable");
    assert_eq!(teacher_notification_count, 1);

    let admin_notifications = notifications::table
        .filter(notifications::user_id.eq(Some(admin.id())))
        .filter(notifications::title.eq("reward_fraud_block:created"))
        .load::<Notification>(&mut conn)
        .await
        .expect("platform reviewer fraud block notifications should load");
    assert_eq!(admin_notifications.len(), 1);
    assert!(admin_notifications[0]
        .body
        .contains("suspicious reward approvals"));

    let revoked = revoke_reward_fraud_block(&mut conn, admin.id(), teacher_block.id)
        .await
        .expect("admin should revoke teacher reward fraud block");
    assert_eq!(revoked.revoked_by_user_id, Some(admin.id()));
    assert!(revoked.revoked_at.is_some());
    let teacher_revoked_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(teacher.id())))
        .filter(notifications::title.eq("reward_fraud_block:revoked"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("teacher revoked fraud block notifications should be countable");
    assert_eq!(teacher_revoked_notification_count, 1);

    let organization_block = create_reward_fraud_block(
        &mut conn,
        admin.id(),
        RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION.to_string(),
            teacher_user_id: None,
            organization_id: Some(organization.id),
            course_id: None,
            reward_policy_id: None,
            reason: "organization reward submissions paused".to_string(),
            evidence_reference: None,
            expires_at: None,
        },
    )
    .await
    .expect("admin should block organization reward activity");
    assert_eq!(
        organization_block.scope_type,
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION
    );
    assert_eq!(organization_block.organization_id, Some(organization.id));
    let organization_operator_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(org_operator.id())))
        .filter(notifications::title.eq("reward_fraud_block:created"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("organization operator fraud block notifications should be countable");
    assert_eq!(organization_operator_notification_count, 1);

    let course_block = create_reward_fraud_block(
        &mut conn,
        admin.id(),
        RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string(),
            teacher_user_id: None,
            organization_id: None,
            course_id: Some(course.id),
            reward_policy_id: None,
            reason: "course reward rules under review".to_string(),
            evidence_reference: Some("case://course-block".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should block course reward activity through fraud management permission");
    assert_eq!(course_block.scope_type, REWARD_FRAUD_BLOCK_SCOPE_COURSE);
    assert_eq!(course_block.course_id, Some(course.id));

    let stored = reward_fraud_blocks::table
        .find(course_block.id)
        .first::<RewardFraudBlock>(&mut conn)
        .await
        .expect("course reward fraud block should be persisted");
    assert_eq!(stored.created_by_user_id, admin.id());
    assert_eq!(stored.reason, "course reward rules under review");
}
