use crate::support::*;

pub(crate) async fn create_platform_delegated_permission(
    conn: &mut AsyncPgConnection,
    grantor_user_id: i32,
    grantee_user_id: i32,
) -> i64 {
    diesel::insert_into(delegated_permissions::table)
        .values(NewDelegatedPermission {
            grantor_user_id,
            grantee_user_id,
            permission: Permissions::APPROVE_REWARD_AMOUNT.to_string(),
            scope_type: DELEGATED_SCOPE_PLATFORM.to_string(),
            organization_id: None,
            course_id: None,
            reason: Some("export delegated permission activity".to_string()),
            expires_at: None,
        })
        .returning(delegated_permissions::id)
        .get_result(conn)
        .await
        .expect("failed to create delegated permission")
}

pub(crate) async fn create_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    creator_user_id: i32,
) -> i64 {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: 1,
            token_amount: BigDecimal::from(10),
            multiplier: BigDecimal::from(1),
            max_payout: Some(BigDecimal::from(100)),
            cooldown_seconds: 0,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: Some(creator_user_id),
        })
        .returning(reward_policies::id)
        .get_result(conn)
        .await
        .expect("failed to create reward policy")
}

pub(crate) struct FraudBlockSeed<'a> {
    pub(crate) created_by_user_id: i32,
    pub(crate) scope_type: &'a str,
    pub(crate) teacher_user_id: Option<i32>,
    pub(crate) organization_id: Option<i32>,
    pub(crate) course_id: Option<i32>,
    pub(crate) reward_policy_id: Option<i64>,
    pub(crate) reason: &'a str,
    pub(crate) expires_at: Option<chrono::DateTime<Utc>>,
}

pub(crate) async fn create_fraud_block(
    conn: &mut AsyncPgConnection,
    seed: FraudBlockSeed<'_>,
) -> i64 {
    diesel::insert_into(reward_fraud_blocks::table)
        .values(NewRewardFraudBlock {
            scope_type: seed.scope_type.to_string(),
            teacher_user_id: seed.teacher_user_id,
            organization_id: seed.organization_id,
            course_id: seed.course_id,
            reward_policy_id: seed.reward_policy_id,
            reason: seed.reason.to_string(),
            evidence_reference: Some(format!("case://{}", unique_string("fraud_report"))),
            created_by_user_id: seed.created_by_user_id,
            expires_at: seed.expires_at,
        })
        .returning(reward_fraud_blocks::id)
        .get_result(conn)
        .await
        .expect("failed to create fraud block")
}
