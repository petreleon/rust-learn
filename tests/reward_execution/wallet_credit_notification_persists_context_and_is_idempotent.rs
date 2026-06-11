#[actix_web::test]
async fn wallet_credit_notification_persists_context_and_is_idempotent() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("RewardNotificationCourse")).await;
    let student = create_user_helper(&mut conn, "reward_notification_student").await;
    let submitter = create_user_helper(&mut conn, "reward_notification_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_CONFIRMED,
        Some(BigDecimal::from(21)),
    )
    .await;

    let credited = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect("token-confirmed candidate should credit wallet");
    let transaction_id = credited
        .transaction_id
        .expect("wallet credit should create a transaction");

    let sent = notify_reward_wallet_credit(&mut conn, candidate.id)
        .await
        .expect("wallet credited candidate should notify the student");
    assert!(sent.notified);
    assert_eq!(sent.wallet_id, credited.wallet_id);
    assert_eq!(sent.transaction_id, transaction_id);
    assert_eq!(sent.amount, BigDecimal::from(21));

    let notification_id = sent
        .notification_id
        .expect("notification result should include notification id");
    let notification = notifications::table
        .find(notification_id)
        .first::<rust_learn::models::notification::Notification>(&mut conn)
        .await
        .expect("reward wallet notification should exist");
    assert_eq!(notification.user_id, Some(student.id()));
    assert_eq!(notification.title, "reward:wallet_credited");
    assert!(notification
        .body
        .contains(&format!("course #{}", course.id)));
    assert!(notification.body.contains(course.title.as_str()));
    assert!(notification.body.contains("21 LearnToken"));
    assert!(notification
        .body
        .contains(&format!("wallet #{}", credited.wallet_id)));
    assert!(notification
        .body
        .contains(&format!("Transaction #{}", transaction_id)));

    let credit_record = reward_wallet_credit_records::table
        .find(
            credited
                .credit_record_id
                .expect("wallet credit should create record"),
        )
        .first::<rust_learn::models::reward_wallet_credit_record::RewardWalletCreditRecord>(
            &mut conn,
        )
        .await
        .expect("reward wallet credit record should exist");
    assert_eq!(credit_record.notification_id, Some(notification_id));
    assert!(credit_record.notified_at.is_some());

    let candidate_status = reward_candidates::table
        .find(candidate.id)
        .select(reward_candidates::status)
        .first::<String>(&mut conn)
        .await
        .expect("candidate status should be queryable");
    assert_eq!(candidate_status, REWARD_STATUS_NOTIFIED);

    let duplicate = notify_reward_wallet_credit(&mut conn, candidate.id)
        .await
        .expect("duplicate wallet credit notification should be idempotent");
    assert!(!duplicate.notified);
    assert_eq!(duplicate.notification_id, Some(notification_id));
    assert_eq!(duplicate.transaction_id, transaction_id);

    let notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(student.id())))
        .filter(notifications::title.eq("reward:wallet_credited"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("reward wallet notifications should be countable");
    assert_eq!(notification_count, 1);

    let audit = list_reward_audit_events(&mut conn, candidate.id)
        .await
        .expect("wallet credit audit events should load");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[0].event_type, REWARD_AUDIT_EVENT_WALLET_CREDITED);
    assert_eq!(
        audit[0].from_status.as_deref(),
        Some(REWARD_STATUS_TOKEN_CONFIRMED)
    );
    assert_eq!(audit[0].to_status, REWARD_STATUS_WALLET_CREDITED);
    assert_eq!(
        audit[1].event_type,
        REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED
    );
    assert_eq!(
        audit[1].from_status.as_deref(),
        Some(REWARD_STATUS_WALLET_CREDITED)
    );
    assert_eq!(audit[1].to_status, REWARD_STATUS_NOTIFIED);
}
