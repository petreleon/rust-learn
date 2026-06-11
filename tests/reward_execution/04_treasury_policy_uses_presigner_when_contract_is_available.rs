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
