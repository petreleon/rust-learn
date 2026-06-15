use crate::{
    create_platform_delegated_permission::*, platform_fraud_dashboard_helpers::*, support::*,
};

#[actix_web::test]
async fn platform_fraud_dashboard_reports_active_blocks_by_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_test_user(&mut conn, "report_fraud_admin").await;
    let platform_moderator = create_test_user(&mut conn, "report_fraud_moderator").await;
    let stranger = create_test_user(&mut conn, "report_fraud_stranger").await;
    let teacher = create_test_user(&mut conn, "report_fraud_teacher").await;
    let expired_teacher = create_test_user(&mut conn, "report_fraud_expired_teacher").await;
    assign_platform_role(&mut conn, platform_admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, platform_moderator.id(), "MODERATOR").await;
    let org = create_organization(&mut conn).await;
    let course = create_course(&mut conn).await;
    let policy_id = create_course_reward_policy(&mut conn, course.id, platform_admin.id()).await;

    let teacher_block_id = create_fraud_block(
        &mut conn,
        FraudBlockSeed {
            created_by_user_id: platform_admin.id(),
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
            teacher_user_id: Some(teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "teacher approvals paused for review",
            expires_at: None,
        },
    )
    .await;
    let organization_block_id = create_fraud_block(
        &mut conn,
        FraudBlockSeed {
            created_by_user_id: platform_admin.id(),
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
            teacher_user_id: None,
            organization_id: Some(org.id),
            course_id: None,
            reward_policy_id: None,
            reason: "organization reward activity paused",
            expires_at: None,
        },
    )
    .await;
    let course_block_id = create_fraud_block(
        &mut conn,
        FraudBlockSeed {
            created_by_user_id: platform_admin.id(),
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_COURSE,
            teacher_user_id: None,
            organization_id: None,
            course_id: Some(course.id),
            reward_policy_id: None,
            reason: "course rewards under review",
            expires_at: None,
        },
    )
    .await;
    let policy_block_id = create_fraud_block(
        &mut conn,
        FraudBlockSeed {
            created_by_user_id: platform_admin.id(),
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
            teacher_user_id: None,
            organization_id: None,
            course_id: None,
            reward_policy_id: Some(policy_id),
            reason: "policy payouts paused",
            expires_at: None,
        },
    )
    .await;
    let expired_block_id = create_fraud_block(
        &mut conn,
        FraudBlockSeed {
            created_by_user_id: platform_admin.id(),
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
            teacher_user_id: Some(expired_teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "expired teacher block",
            expires_at: Some(Utc::now() - Duration::minutes(5)),
        },
    )
    .await;
    drop(conn);

    assert_platform_fraud_dashboard(
        &pool,
        PlatformFraudDashboardExpectation {
            admin_id: platform_admin.id(),
            moderator_id: platform_moderator.id(),
            stranger_id: stranger.id(),
            active_block_ids: [
                teacher_block_id,
                organization_block_id,
                course_block_id,
                policy_block_id,
            ],
            expired_block_id,
        },
    )
    .await;
}
