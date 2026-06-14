#[actix_web::test]
async fn platform_csv_exports_cover_business_reward_datasets() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_test_user(&mut conn, "report_export_admin").await;
    let platform_moderator = create_test_user(&mut conn, "report_export_moderator").await;
    let applicant = create_test_user(&mut conn, "report_export_applicant").await;
    let student = create_test_user(&mut conn, "report_export_student").await;
    let submitter = create_test_user(&mut conn, "report_export_submitter").await;
    assign_platform_role(&mut conn, platform_admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, platform_moderator.id(), "MODERATOR").await;
    let org = create_organization(&mut conn).await;
    let wallet_id = create_org_wallet_with_value(&mut conn, org.id, BigDecimal::from(10)).await;
    let course = create_course(&mut conn).await;
    let application_id = create_teacher_application(&mut conn, applicant.id()).await;
    let candidate_id = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
    )
    .await;
    mark_reward_approval_decisions(&mut conn, candidate_id, submitter.id(), platform_admin.id())
        .await;
    let (payout_record_id, external_transaction_id, transaction_hash) =
        create_reward_payout_export_records(&mut conn, candidate_id, BigDecimal::from(10)).await;
    let wallet_credit_record_id = create_reward_wallet_credit_export_record(
        &mut conn,
        candidate_id,
        wallet_id,
        BigDecimal::from(10),
    )
    .await;
    let delegation_id =
        create_platform_delegated_permission(&mut conn, platform_admin.id(), submitter.id()).await;
    drop(conn);
    use rust_learn::application::reporting::platform_csv_exports::PlatformCsvExportsUseCase;
    use rust_learn::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationUseCase;
    use rust_learn::infra::postgres::reporting::platform_csv_export_use_case::PostgresPlatformCsvExportsUseCase;
    use rust_learn::infra::postgres::reporting::platform_wallet_reconciliation_use_case::PostgresPlatformWalletReconciliationUseCase;
    let csv_exports_use_case = web::Data::new(
        Arc::new(PostgresPlatformCsvExportsUseCase::new(pool.clone()))
            as Arc<dyn PlatformCsvExportsUseCase>,
    );
    let wallet_reconciliation_use_case = web::Data::new(Arc::new(
        PostgresPlatformWalletReconciliationUseCase::new(pool.clone()),
    ) as Arc<dyn PlatformWalletReconciliationUseCase>);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(csv_exports_use_case)
            .app_data(wallet_reconciliation_use_case)
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::reporting::configure_routes),
    )
    .await;
    for path in [
        "/reports/platform/teacher-applications.csv",
        "/reports/platform/reward-approvals.csv",
        "/reports/platform/token-payouts.csv",
        "/reports/platform/wallet-credits.csv",
        "/reports/platform/delegated-permissions.csv",
        "/reports/platform/wallet-reconciliation",
    ] {
        let denied_req = test::TestRequest::get()
            .uri(path)
            .insert_header((
                "Authorization",
                format!("Bearer {}", token_for(platform_moderator.id())),
            ))
            .to_request();
        let denied_status = match app.call(denied_req).await {
            Ok(resp) => resp.status(),
            Err(err) => err.error_response().status(),
        };
        assert_eq!(denied_status, StatusCode::FORBIDDEN);
    }

    let teacher_applications_req = test::TestRequest::get()
        .uri("/reports/platform/teacher-applications.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let teacher_applications_resp = test::call_service(&app, teacher_applications_req).await;
    assert_eq!(teacher_applications_resp.status(), StatusCode::OK);
    let teacher_applications_csv =
        String::from_utf8(test::read_body(teacher_applications_resp).await.to_vec())
            .expect("valid teacher applications csv");
    assert!(teacher_applications_csv.starts_with("application_id,applicant_user_id"));
    assert!(teacher_applications_csv.contains(&format!("{application_id},")));

    let reward_approvals_req = test::TestRequest::get()
        .uri("/reports/platform/reward-approvals.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let reward_approvals_resp = test::call_service(&app, reward_approvals_req).await;
    assert_eq!(reward_approvals_resp.status(), StatusCode::OK);
    let reward_approvals_csv =
        String::from_utf8(test::read_body(reward_approvals_resp).await.to_vec())
            .expect("valid reward approvals csv");
    assert!(reward_approvals_csv.starts_with("reward_candidate_id,course_id"));
    assert!(reward_approvals_csv.contains(&format!("{candidate_id},")));
    assert!(reward_approvals_csv.contains("course reward approved"));
    assert!(reward_approvals_csv.contains("platform amount approved"));

    let token_payouts_req = test::TestRequest::get()
        .uri("/reports/platform/token-payouts.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let token_payouts_resp = test::call_service(&app, token_payouts_req).await;
    assert_eq!(token_payouts_resp.status(), StatusCode::OK);
    let token_payouts_csv = String::from_utf8(test::read_body(token_payouts_resp).await.to_vec())
        .expect("valid token payouts csv");
    assert!(token_payouts_csv.starts_with("reward_payout_record_id,reward_candidate_id"));
    assert!(token_payouts_csv.contains(&format!("{payout_record_id},{candidate_id},")));
    assert!(token_payouts_csv.contains(&external_transaction_id.to_string()));
    assert!(token_payouts_csv.contains(&transaction_hash));

    let wallet_credits_req = test::TestRequest::get()
        .uri("/reports/platform/wallet-credits.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let wallet_credits_resp = test::call_service(&app, wallet_credits_req).await;
    assert_eq!(wallet_credits_resp.status(), StatusCode::OK);
    let wallet_credits_csv = String::from_utf8(test::read_body(wallet_credits_resp).await.to_vec())
        .expect("valid wallet credits csv");
    assert!(wallet_credits_csv.starts_with("reward_wallet_credit_record_id,reward_candidate_id"));
    assert!(wallet_credits_csv.contains(&format!("{wallet_credit_record_id},{candidate_id},")));
    assert!(wallet_credits_csv.contains(&format!(",{wallet_id},")));

    let delegated_permissions_req = test::TestRequest::get()
        .uri("/reports/platform/delegated-permissions.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let delegated_permissions_resp = test::call_service(&app, delegated_permissions_req).await;
    assert_eq!(delegated_permissions_resp.status(), StatusCode::OK);
    let delegated_permissions_csv =
        String::from_utf8(test::read_body(delegated_permissions_resp).await.to_vec())
            .expect("valid delegated permissions csv");
    assert!(delegated_permissions_csv.starts_with("delegated_permission_id,grantor_user_id"));
    assert!(delegated_permissions_csv.contains(&format!("{delegation_id},")));
    assert!(delegated_permissions_csv.contains("APPROVE_REWARD_AMOUNT"));
    assert!(delegated_permissions_csv.contains(",active,"));
    let wallet_reconciliation_req = test::TestRequest::get()
        .uri("/reports/platform/wallet-reconciliation")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let wallet_reconciliation_resp = test::call_service(&app, wallet_reconciliation_req).await;
    assert_eq!(wallet_reconciliation_resp.status(), StatusCode::OK);
    let wallet_reconciliation: Value = test::read_body_json(wallet_reconciliation_resp).await;
    assert!(wallet_reconciliation["total_wallets"].as_i64().unwrap_or_default() >= 1);
    assert!(wallet_reconciliation["total_reward_records"].as_i64().unwrap_or_default() >= 1);
    assert!(
        wallet_reconciliation["wallets"]
            .as_array()
            .expect("wallet rows")
            .iter()
            .any(|wallet| wallet["wallet_id"].as_i64() == Some(i64::from(wallet_id)))
    );
}
