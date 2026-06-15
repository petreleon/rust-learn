use crate::{create_course::*, support::*, use_case_helpers::*};

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
