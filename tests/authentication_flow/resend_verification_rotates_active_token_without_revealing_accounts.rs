use crate::support::*;

#[actix_web::test]
async fn resend_verification_rotates_active_token_without_revealing_accounts() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let email = unique_email("auth-resend-verification");
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "password": "ValidPass123!",
            "name": "Resend Verification",
            "date_of_birth": "2005-10-11"
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), StatusCode::OK);

    let mut conn = setup_conn(&pool).await;
    let user = find_user_by_email(&mut conn, &email)
        .await
        .expect("registered user should exist");
    assert_eq!(active_verification_tokens(&mut conn, user.id()).await, 1);
    drop(conn);

    let resend_resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/auth/resend-verification")
            .set_json(serde_json::json!({ "email": email }))
            .to_request(),
    )
    .await;
    assert_eq!(resend_resp.status(), StatusCode::OK);
    let body = test::read_body(resend_resp).await;
    assert_eq!(
        body.as_ref(),
        b"If an unverified account matches that email, a verification link has been sent."
    );

    let mut conn = setup_conn(&pool).await;
    assert_eq!(active_verification_tokens(&mut conn, user.id()).await, 1);
    assert_eq!(total_verification_tokens(&mut conn, user.id()).await, 2);
    drop(conn);

    let unknown_resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/auth/resend-verification")
            .set_json(serde_json::json!({ "email": unique_email("auth-resend-unknown") }))
            .to_request(),
    )
    .await;
    assert_eq!(unknown_resp.status(), StatusCode::OK);
}

async fn active_verification_tokens(
    conn: &mut diesel_async::AsyncPgConnection,
    user_id: i32,
) -> i64 {
    email_verification_tokens::table
        .filter(email_verification_tokens::user_id.eq(user_id))
        .filter(email_verification_tokens::used_at.is_null())
        .count()
        .get_result(conn)
        .await
        .expect("active token count should load")
}

async fn total_verification_tokens(
    conn: &mut diesel_async::AsyncPgConnection,
    user_id: i32,
) -> i64 {
    email_verification_tokens::table
        .filter(email_verification_tokens::user_id.eq(user_id))
        .count()
        .get_result(conn)
        .await
        .expect("total token count should load")
}
