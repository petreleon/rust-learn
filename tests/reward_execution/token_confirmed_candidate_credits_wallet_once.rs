use crate::{create_course::*, support::*, use_case_helpers::*};

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
    let credit_record_id = credited
        .credit_record_id
        .expect("wallet credit should create credit record");

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

    let credit_record = reward_wallet_credit_records::table
        .find(credit_record_id)
        .first::<rust_learn::models::reward_wallet_credit_record::RewardWalletCreditRecord>(
            &mut conn,
        )
        .await
        .expect("reward wallet credit record should exist");
    assert_eq!(credit_record.reward_candidate_id, candidate.id);
    assert_eq!(credit_record.wallet_id, credited.wallet_id);
    assert_eq!(credit_record.transaction_id, transaction_id);
    assert_eq!(
        credit_record.internal_transaction_id,
        internal_transaction_id
    );
    assert_eq!(credit_record.notification_id, None);

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
    assert_eq!(duplicate.credit_record_id, credited.credit_record_id);
    assert_eq!(duplicate.transaction_id, credited.transaction_id);
    assert_eq!(
        duplicate.internal_transaction_id,
        credited.internal_transaction_id
    );

    let wallet_value_after_duplicate = wallets::table
        .find(credited.wallet_id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should still exist");
    assert_eq!(wallet_value_after_duplicate, BigDecimal::from(15));
}
