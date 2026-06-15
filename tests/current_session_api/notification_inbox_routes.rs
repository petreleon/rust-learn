use crate::{current_session_test_app::*, support::*};

#[actix_web::test]
async fn notification_inbox_routes_list_mark_read_and_clear() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let user = create_test_user(&mut conn, "notification_inbox_user").await;
    let other_user = create_test_user(&mut conn, "notification_inbox_other").await;
    drop(conn);

    let notifications = NotificationsState::new(pool.clone());
    let first_id = notifications
        .send_notification(user.id(), "inbox:first", "First notification")
        .await
        .expect("first notification should persist");
    notifications
        .send_notification(user.id(), "inbox:second", "Second notification")
        .await
        .expect("second notification should persist");
    notifications
        .send_notification(other_user.id(), "inbox:other", "Other notification")
        .await
        .expect("other notification should persist");

    let app = test::init_service(current_session_test_app(pool.clone())).await;
    let auth_header = ("Authorization", format!("Bearer {}", token_for(user.id())));

    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me/notifications")
            .insert_header(auth_header.clone())
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;
    let items = body.as_array().expect("notifications should be an array");
    assert_eq!(items.len(), 2);
    assert!(items.iter().any(|item| item["title"] == "inbox:first"));
    assert!(items.iter().any(|item| item["title"] == "inbox:second"));
    assert!(!items.iter().any(|item| item["title"] == "inbox:other"));

    let response = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&format!("/api/me/notifications/{first_id}/read"))
            .insert_header(auth_header.clone())
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = String::from_utf8(test::read_body(response).await.to_vec())
        .expect("mark-read body should be valid utf8");
    assert_eq!(body, "Notification marked as read");

    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me/notifications")
            .insert_header(auth_header.clone())
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;
    let first = body
        .as_array()
        .and_then(|items| {
            items
                .iter()
                .find(|item| item["id"].as_i64() == Some(first_id))
        })
        .expect("first notification should be returned");
    assert_eq!(first["read"].as_bool(), Some(true));

    let response = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri("/api/me/notifications")
            .insert_header(auth_header.clone())
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = String::from_utf8(test::read_body(response).await.to_vec())
        .expect("clear body should be valid utf8");
    assert_eq!(body, "Notifications cleared");

    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me/notifications")
            .insert_header(auth_header)
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body.as_array().map(Vec::len), Some(0));
}
