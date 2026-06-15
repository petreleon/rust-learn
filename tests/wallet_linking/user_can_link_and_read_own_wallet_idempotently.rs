use crate::http_support::{assign_platform_permission_role, token_for, wallet_test_app};
use crate::support::*;

#[actix_web::test]
async fn user_can_link_and_read_own_wallet_idempotently() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let user = create_test_user(&mut conn, "wallet_self").await;
    drop(conn);

    let app = test::init_service(wallet_test_app(pool.clone())).await;
    let token = token_for(user.id());

    let blocked_req = test::TestRequest::post()
        .uri("/api/wallets/me/link")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let blocked_resp = test::call_service(&app, blocked_req).await;
    assert_eq!(blocked_resp.status(), StatusCode::CONFLICT);
    let blocked_body: Value = test::read_body_json(blocked_resp).await;
    assert_eq!(blocked_body["error"]["code"], "kyc_required");
    assert_eq!(
        blocked_body["error"]["message"],
        "KYC verification is required before wallet actions"
    );
    assert_eq!(blocked_body["error"]["status"], 409);

    let mut conn = setup_conn(&pool).await;
    mark_user_kyc_verified(&mut conn, user.id()).await;
    drop(conn);

    let link_req = test::TestRequest::post()
        .uri("/api/wallets/me/link")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let link_resp = test::call_service(&app, link_req).await;
    assert_eq!(link_resp.status(), StatusCode::CREATED);
    let first: Value = test::read_body_json(link_resp).await;
    assert_eq!(first["created"], true);
    assert_eq!(first["wallet"]["owner_type"], "user");
    assert_eq!(first["wallet"]["user_id"], user.id());
    assert!(first["wallet"]["organization_id"].is_null());
    assert_eq!(first["wallet"]["value"], "0");
    let wallet_id = first["wallet"]["id"].as_i64().expect("wallet id");

    let duplicate_req = test::TestRequest::post()
        .uri("/api/wallets/me/link")
        .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
        .to_request();
    let duplicate_resp = test::call_service(&app, duplicate_req).await;
    assert_eq!(duplicate_resp.status(), StatusCode::OK);
    let duplicate: Value = test::read_body_json(duplicate_resp).await;
    assert_eq!(duplicate["created"], false);
    assert_eq!(duplicate["wallet"]["id"], wallet_id);

    let get_req = test::TestRequest::get()
        .uri("/api/wallets/me")
        .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
        .to_request();
    let get_resp = test::call_service(&app, get_req).await;
    assert_eq!(get_resp.status(), StatusCode::OK);
    let fetched: Value = test::read_body_json(get_resp).await;
    assert_eq!(fetched["id"], wallet_id);
    assert_eq!(fetched["user_id"], user.id());
}

#[actix_web::test]
async fn platform_wallet_manager_can_link_another_user_wallet() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let manager = create_test_user(&mut conn, "wallet_manager").await;
    let stranger = create_test_user(&mut conn, "wallet_stranger").await;
    let auditor = create_test_user(&mut conn, "wallet_auditor").await;
    let target = create_test_user(&mut conn, "wallet_target").await;
    let second_target = create_test_user(&mut conn, "wallet_target_two").await;
    mark_user_kyc_verified(&mut conn, target.id()).await;
    mark_user_kyc_verified(&mut conn, second_target.id()).await;
    assign_platform_role(&mut conn, manager.id(), "ADMIN").await;
    assign_platform_role(&mut conn, stranger.id(), "MODERATOR").await;
    assign_platform_permission_role(&mut conn, auditor.id(), Permissions::VIEW_TRANSACTIONS).await;
    drop(conn);

    let app = test::init_service(wallet_test_app(pool.clone())).await;

    let forbidden_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/users/{}/link", target.id()))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_resp = test::call_service(&app, forbidden_req).await;
    assert_eq!(forbidden_resp.status(), StatusCode::FORBIDDEN);

    let forbidden_get_req = test::TestRequest::get()
        .uri(&format!("/api/wallets/users/{}", target.id()))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_get_resp = test::call_service(&app, forbidden_get_req).await;
    assert_eq!(forbidden_get_resp.status(), StatusCode::FORBIDDEN);

    let manager_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/users/{}/link", target.id()))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(manager.id())),
        ))
        .to_request();
    let manager_resp = test::call_service(&app, manager_req).await;
    assert_eq!(manager_resp.status(), StatusCode::CREATED);
    let linked: Value = test::read_body_json(manager_resp).await;
    assert_eq!(linked["created"], true);
    assert_eq!(linked["wallet"]["user_id"], target.id());

    let auditor_get_req = test::TestRequest::get()
        .uri(&format!("/api/wallets/users/{}", target.id()))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(auditor.id())),
        ))
        .to_request();
    let auditor_get_resp = test::call_service(&app, auditor_get_req).await;
    assert_eq!(auditor_get_resp.status(), StatusCode::OK);
    let audited: Value = test::read_body_json(auditor_get_resp).await;
    assert_eq!(audited["user_id"], target.id());

    let auditor_link_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/users/{}/link", second_target.id()))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(auditor.id())),
        ))
        .to_request();
    let auditor_link_resp = test::call_service(&app, auditor_link_req).await;
    assert_eq!(auditor_link_resp.status(), StatusCode::FORBIDDEN);
}
