use crate::{create_course::*, support::*, use_case_helpers::*};

#[actix_web::test]
async fn token_confirmation_records_external_transaction_and_candidate_link() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("TokenConfirmationCourse")).await;
    let student = create_user_helper(&mut conn, "token_confirmation_student").await;
    let submitter = create_user_helper(&mut conn, "token_confirmation_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_PENDING,
        Some(BigDecimal::from(17)),
    )
    .await;

    let request = RewardTokenConfirmationRequest {
        chain_id: 31337,
        contract_address: "0x00000000000000000000000000000000000000cc".to_string(),
        transaction_hash: unique_hash("reward"),
        log_index: 3,
        event_type: "transfer".to_string(),
        from_address: Some("0x00000000000000000000000000000000000000dd".to_string()),
        to_address: "0x00000000000000000000000000000000000000ee".to_string(),
        amount: BigDecimal::from(17),
    };

    let confirmation = record_reward_token_confirmation(&mut conn, candidate.id, request.clone())
        .await
        .expect("token-pending candidate should record token confirmation");
    assert!(confirmation.inserted_external_transaction);

    let external = external_transactions::table
        .find(confirmation.external_transaction_id)
        .first::<rust_learn::models::transaction::ExternalTransaction>(&mut conn)
        .await
        .expect("external transaction should exist");
    assert_eq!(external.chain_id, Some(request.chain_id));
    assert_eq!(
        external.contract_address.as_deref(),
        Some(request.contract_address.as_str())
    );
    assert_eq!(
        external.transaction_hash.as_deref(),
        Some(request.transaction_hash.as_str())
    );
    assert_eq!(external.log_index, Some(request.log_index));
    assert_eq!(external.event_type.as_deref(), Some("transfer"));
    assert_eq!(
        external.from_address.as_deref(),
        request.from_address.as_deref()
    );
    assert_eq!(
        external.to_address.as_deref(),
        Some(request.to_address.as_str())
    );
    assert_eq!(external.amount, request.amount);

    let payout_record = reward_payout_records::table
        .find(confirmation.payout_record_id)
        .first::<rust_learn::models::reward_payout_record::RewardPayoutRecord>(&mut conn)
        .await
        .expect("reward payout record should exist");
    assert_eq!(payout_record.reward_candidate_id, candidate.id);
    assert_eq!(
        payout_record.external_transaction_id,
        confirmation.external_transaction_id
    );
    assert_eq!(payout_record.transaction_id, confirmation.transaction_id);

    let transaction_link_count = transactions_external_transactions::table
        .filter(transactions_external_transactions::transaction_id.eq(confirmation.transaction_id))
        .filter(
            transactions_external_transactions::external_transaction_id
                .eq(confirmation.external_transaction_id),
        )
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("external transaction link should be queryable");
    assert_eq!(transaction_link_count, 1);

    let candidate_status = reward_candidates::table
        .find(candidate.id)
        .select(reward_candidates::status)
        .first::<String>(&mut conn)
        .await
        .expect("candidate status should be queryable");
    assert_eq!(candidate_status, REWARD_STATUS_TOKEN_CONFIRMED);

    let duplicate = record_reward_token_confirmation(&mut conn, candidate.id, request)
        .await
        .expect("duplicate token confirmation should be idempotent");
    assert!(!duplicate.inserted_external_transaction);
    assert_eq!(duplicate.transaction_id, confirmation.transaction_id);
    assert_eq!(
        duplicate.external_transaction_id,
        confirmation.external_transaction_id
    );
    assert_eq!(duplicate.payout_record_id, confirmation.payout_record_id);

    let audit = list_reward_audit_events(&mut conn, candidate.id)
        .await
        .expect("token confirmation audit events should load");
    assert_eq!(
        audit.len(),
        1,
        "duplicate token confirmation must not create a second audit transition"
    );
    assert_eq!(audit[0].event_type, REWARD_AUDIT_EVENT_TOKEN_CONFIRMED);
    assert_eq!(
        audit[0].from_status.as_deref(),
        Some(REWARD_STATUS_TOKEN_PENDING)
    );
    assert_eq!(audit[0].to_status, REWARD_STATUS_TOKEN_CONFIRMED);
}
