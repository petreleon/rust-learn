use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, reward_candidates, reward_policies};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, RewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_MINT, REWARD_PAYMENT_TREASURY_TRANSFER,
    REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::user::User;
use rust_learn::repositories::persistent_state_repository::set_persistent_state;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_execution_service::{
    plan_reward_payout, RewardExecutionError, REWARD_PAYOUT_METHOD_MINT,
    REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER,
};
use serde_json::json;

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

async fn create_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    payment_strategy: &str,
) {
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
            payment_strategy: payment_strategy.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .execute(conn)
        .await
        .expect("failed to create reward policy");
}

async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
    approved_amount: Option<BigDecimal>,
) -> RewardCandidate {
    let candidate = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("reward_execution_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .get_result::<RewardCandidate>(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate.id))
        .set((
            reward_candidates::approved_amount.eq(approved_amount),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter_user_id)),
            reward_candidates::amount_decided_at.eq(Some(Utc::now())),
            reward_candidates::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
        .expect("failed to update reward candidate amount")
}

#[actix_web::test]
async fn treasury_policy_uses_presigner_when_contract_is_available() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("TreasuryPresignerCourse")).await;
    let student = create_user_helper(&mut conn, "treasury_presigner_student").await;
    let submitter = create_user_helper(&mut conn, "treasury_presigner_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;
    set_persistent_state(
        &mut conn,
        "learn_token_presigner_address",
        "0x00000000000000000000000000000000000000aa",
    )
    .await
    .expect("failed to set presigner address");

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(10)),
    )
    .await;

    let plan = plan_reward_payout(&mut conn, candidate.id)
        .await
        .expect("amount-approved candidate should produce a payout plan");

    assert_eq!(plan.payment_strategy, REWARD_PAYMENT_TREASURY_TRANSFER);
    assert_eq!(plan.payout_method, REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER);
    assert!(plan.requires_token_confirmation);
    assert_eq!(plan.amount, BigDecimal::from(10));
}

#[actix_web::test]
async fn mint_method_requires_explicit_mint_policy() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("ExplicitMintCourse")).await;
    let student = create_user_helper(&mut conn, "explicit_mint_student").await;
    let submitter = create_user_helper(&mut conn, "explicit_mint_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_MINT).await;
    set_persistent_state(
        &mut conn,
        "learn_token_presigner_address",
        "0x00000000000000000000000000000000000000bb",
    )
    .await
    .expect("failed to set presigner address");

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(12)),
    )
    .await;

    let plan = plan_reward_payout(&mut conn, candidate.id)
        .await
        .expect("mint policy should produce a mint payout plan");

    assert_eq!(plan.payment_strategy, REWARD_PAYMENT_MINT);
    assert_eq!(plan.payout_method, REWARD_PAYOUT_METHOD_MINT);
    assert!(plan.requires_token_confirmation);
}

#[actix_web::test]
async fn candidate_must_be_amount_approved_before_payout_planning() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("PendingPayoutCourse")).await;
    let student = create_user_helper(&mut conn, "pending_payout_student").await;
    let submitter = create_user_helper(&mut conn, "pending_payout_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        None,
    )
    .await;

    let denied = plan_reward_payout(&mut conn, candidate.id)
        .await
        .expect_err("candidate must be amount approved before payout planning");

    assert!(matches!(denied, RewardExecutionError::InvalidStatus(_)));
}
