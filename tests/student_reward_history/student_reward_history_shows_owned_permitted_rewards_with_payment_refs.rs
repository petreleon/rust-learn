#[actix_web::test]
async fn student_reward_history_shows_owned_permitted_rewards_with_payment_refs() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let student = create_verified_user(&mut conn, "student_reward_history_student").await;
    let other_student = create_verified_user(&mut conn, "student_reward_history_other").await;
    let submitter = create_verified_user(&mut conn, "student_reward_history_submitter").await;
    let course = create_course(&mut conn, "StudentRewardHistoryCourse").await;
    let hidden_course = create_course(&mut conn, "StudentRewardHistoryHiddenCourse").await;
    assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    assign_course_role(&mut conn, other_student.id(), course.id, "STUDENT").await;
    let wallet = wallet_service::link_user_wallet(&mut conn, student.id())
        .await
        .expect("student wallet should link")
        .wallet;

    let amount = BigDecimal::from(12);
    let visible_candidate_id = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_WALLET_CREDITED,
        amount.clone(),
    )
    .await;
    let financials =
        create_reward_financial_records(&mut conn, visible_candidate_id, &wallet, amount.clone())
            .await;
    let other_student_candidate_id = create_reward_candidate(
        &mut conn,
        course.id,
        other_student.id(),
        submitter.id(),
        REWARD_STATUS_WALLET_CREDITED,
        amount.clone(),
    )
    .await;
    let hidden_candidate_id = create_reward_candidate(
        &mut conn,
        hidden_course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_WALLET_CREDITED,
        amount.clone(),
    )
    .await;
    let filtered_candidate_id = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        amount,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(student_reward_history_use_case(&pool)))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::rewards::student_reward_history_resource()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/reward-candidates/me/history?status=wallet_credited")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let history: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(history.len(), 1);
    let row = &history[0];
    assert_eq!(
        row["reward_candidate_id"].as_i64(),
        Some(visible_candidate_id)
    );
    assert_eq!(row["course_id"].as_i64(), Some(i64::from(course.id)));
    assert_eq!(row["course_title"].as_str(), Some(course.title.as_str()));
    assert_eq!(row["status"], REWARD_STATUS_WALLET_CREDITED);
    assert_eq!(row["approved_amount"], "12");
    assert_ne!(
        row["reward_candidate_id"].as_i64(),
        Some(other_student_candidate_id)
    );
    assert_ne!(
        row["reward_candidate_id"].as_i64(),
        Some(hidden_candidate_id)
    );
    assert_ne!(
        row["reward_candidate_id"].as_i64(),
        Some(filtered_candidate_id)
    );

    let wallet_credit = &row["wallet_credit"];
    assert_eq!(
        wallet_credit["reward_wallet_credit_record_id"].as_i64(),
        Some(financials.wallet_credit_record_id)
    );
    assert_eq!(
        wallet_credit["wallet_id"].as_i64(),
        Some(i64::from(wallet.id))
    );
    assert_eq!(
        wallet_credit["transaction_id"].as_i64(),
        Some(financials.wallet_transaction_id)
    );
    assert_eq!(
        wallet_credit["internal_transaction_id"].as_i64(),
        Some(financials.internal_transaction_id)
    );
    assert_eq!(wallet_credit["amount"], "12");

    let token_transaction = &row["token_transaction"];
    assert_eq!(
        token_transaction["payout_transaction_id"].as_i64(),
        Some(financials.payout_transaction_id)
    );
    assert_eq!(
        token_transaction["external_transaction_id"].as_i64(),
        Some(financials.external_transaction_id)
    );
    assert_eq!(token_transaction["amount"], "12");
    assert_eq!(token_transaction["chain_id"].as_i64(), Some(31337));
    assert_eq!(token_transaction["event_type"], "Transfer");
    assert_eq!(
        token_transaction["transaction_hash"].as_str(),
        Some(financials.transaction_hash.as_str())
    );

    let invalid_req = test::TestRequest::get()
        .uri("/reward-candidates/me/history?status=not-a-real-status")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let invalid_resp = test::call_service(&app, invalid_req).await;
    assert_eq!(invalid_resp.status(), StatusCode::BAD_REQUEST);
}
