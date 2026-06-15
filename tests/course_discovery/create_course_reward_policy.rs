use crate::support::*;

pub(crate) async fn create_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: event_type.to_string(),
            version: 1,
            token_amount: BigDecimal::from(25),
            multiplier: BigDecimal::from(1),
            max_payout: None,
            cooldown_seconds: 0,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .execute(conn)
        .await
        .expect("failed to create reward policy");
}

pub(crate) async fn create_pending_join_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> i64 {
    diesel::insert_into(course_join_requests::table)
        .values(NewCourseJoinRequest {
            course_id,
            requester_user_id: user_id,
            status: COURSE_JOIN_STATUS_PENDING.to_string(),
        })
        .returning(course_join_requests::id)
        .get_result(conn)
        .await
        .expect("failed to create join request")
}
