use crate::{current_session_test_app::*, support::*};

#[actix_web::test]
async fn notification_preferences_default_and_save_round_trip() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let user = create_test_user(&mut conn, "notification_preferences_user").await;
    drop(conn);

    let app = test::init_service(current_session_test_app(pool.clone())).await;
    let auth_header = ("Authorization", format!("Bearer {}", token_for(user.id())));

    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me/preferences")
            .insert_header(auth_header.clone())
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["user_id"].as_i64(), Some(user.id() as i64));
    assert_eq!(body["email_enabled"].as_bool(), Some(true));
    assert_eq!(body["push_enabled"].as_bool(), Some(false));
    assert!(body.get("updated_at").is_none());

    let response = test::call_service(
        &app,
        test::TestRequest::put()
            .uri("/api/me/preferences")
            .insert_header(auth_header.clone())
            .set_json(serde_json::json!({
                "email_enabled": false,
                "push_enabled": true
            }))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let saved: Value = test::read_body_json(response).await;
    assert_eq!(saved["user_id"].as_i64(), Some(user.id() as i64));
    assert_eq!(saved["email_enabled"].as_bool(), Some(false));
    assert_eq!(saved["push_enabled"].as_bool(), Some(true));
    assert!(saved["updated_at"].as_str().is_some());

    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me/preferences")
            .insert_header(auth_header)
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let reloaded: Value = test::read_body_json(response).await;
    assert_eq!(reloaded["user_id"].as_i64(), Some(user.id() as i64));
    assert_eq!(reloaded["email_enabled"].as_bool(), Some(false));
    assert_eq!(reloaded["push_enabled"].as_bool(), Some(true));
    assert!(reloaded["updated_at"].as_str().is_some());
}
