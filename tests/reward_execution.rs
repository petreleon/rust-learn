use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{
    courses, internal_transactions, reward_candidates, reward_policies, transactions,
    transactions_internal_transactions, wallets,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, RewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_WALLET_CREDITED,
};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN,
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::user::User;
use rust_learn::repositories::persistent_state_repository::set_persistent_state;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_execution_service::{
    credit_reward_wallet, plan_reward_payout, RewardExecutionError, REWARD_PAYOUT_METHOD_MINT,
    REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER, REWARD_TRANSACTION_TYPE_WALLET_CREDIT,
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

#[actix_web::test]
async fn token_confirmed_candidate_credits_wallet_once() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("TokenConfirmedCreditCourse")).await;
    let student = create_user_helper(&mut conn, "token_confirmed_credit_student").await;
    let submitter = create_user_helper(&mut conn, "token_confirmed_credit_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_CONFIRMED,
        Some(BigDecimal::from(15)),
    )
    .await;

    let credited = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect("token-confirmed candidate should credit wallet");
    assert!(credited.credited);
    assert_eq!(credited.amount, BigDecimal::from(15));

    let wallet_value = wallets::table
        .find(credited.wallet_id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should exist");
    assert_eq!(wallet_value, BigDecimal::from(15));

    let transaction_id = credited
        .transaction_id
        .expect("wallet credit should create transaction");
    let internal_transaction_id = credited
        .internal_transaction_id
        .expect("wallet credit should create internal transaction");
    let transaction_type = transactions::table
        .find(transaction_id)
        .select(transactions::type_)
        .first::<String>(&mut conn)
        .await
        .expect("wallet credit transaction should exist");
    assert_eq!(transaction_type, REWARD_TRANSACTION_TYPE_WALLET_CREDIT);

    let internal_amount = internal_transactions::table
        .find(internal_transaction_id)
        .select(internal_transactions::amount)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet credit internal transaction should exist");
    assert_eq!(internal_amount, BigDecimal::from(15));

    let link_count = transactions_internal_transactions::table
        .filter(transactions_internal_transactions::transaction_id.eq(transaction_id))
        .filter(
            transactions_internal_transactions::internal_transaction_id.eq(internal_transaction_id),
        )
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("wallet credit transaction link should be queryable");
    assert_eq!(link_count, 1);

    let candidate_status = reward_candidates::table
        .find(candidate.id)
        .select(reward_candidates::status)
        .first::<String>(&mut conn)
        .await
        .expect("candidate status should be queryable");
    assert_eq!(candidate_status, REWARD_STATUS_WALLET_CREDITED);

    let duplicate = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect("wallet credit should be idempotent after status update");
    assert!(!duplicate.credited);
    assert_eq!(duplicate.transaction_id, None);
    assert_eq!(duplicate.internal_transaction_id, None);

    let wallet_value_after_duplicate = wallets::table
        .find(credited.wallet_id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should still exist");
    assert_eq!(wallet_value_after_duplicate, BigDecimal::from(15));
}

#[actix_web::test]
async fn token_policy_cannot_credit_wallet_before_token_confirmation() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("BlockedCreditCourse")).await;
    let student = create_user_helper(&mut conn, "blocked_credit_student").await;
    let submitter = create_user_helper(&mut conn, "blocked_credit_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(11)),
    )
    .await;

    let denied = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect_err("token payout should not credit wallet before token confirmation");
    assert!(matches!(denied, RewardExecutionError::InvalidStatus(_)));
}

#[actix_web::test]
async fn off_chain_policy_can_credit_wallet_after_amount_approval() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("OffChainCreditCourse")).await;
    let student = create_user_helper(&mut conn, "off_chain_credit_student").await;
    let submitter = create_user_helper(&mut conn, "off_chain_credit_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_OFF_CHAIN).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(9)),
    )
    .await;

    let credited = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect("off-chain policy should credit wallet after amount approval");
    assert!(credited.credited);
    assert_eq!(credited.amount, BigDecimal::from(9));
}
