use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel_async::AsyncPgConnection;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::role::PlatformRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::{json, Value};

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
    let email = format!("{}@example.com", unique_string(name));
    create_user(
        conn,
        name,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

#[actix_web::test]
async fn fraud_block_api_separates_read_audit_from_block_management() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "fraud_api_admin").await;
    let moderator = create_test_user(&mut conn, "fraud_api_moderator").await;
    let teacher = create_test_user(&mut conn, "fraud_api_teacher").await;
    assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, moderator.id(), "MODERATOR").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reward_fraud_blocks::reward_fraud_block_scope()),
    )
    .await;

    let block_body = json!({
        "scope_type": "teacher",
        "teacher_user_id": teacher.id(),
        "reason": "api suspicious reward approvals",
        "evidence_reference": "case://api-teacher-block"
    });

    let denied_req = test::TestRequest::post()
        .uri("/reward-fraud-blocks")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .set_json(&block_body)
        .to_request();
    let denied_resp = test::call_service(&app, denied_req).await;
    assert_eq!(denied_resp.status(), StatusCode::FORBIDDEN);

    let create_req = test::TestRequest::post()
        .uri("/reward-fraud-blocks")
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .set_json(&block_body)
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let created: Value = test::read_body_json(create_resp).await;
    let block_id = created["id"].as_i64().expect("created fraud block id");
    assert_eq!(created["scope_type"], "teacher");
    assert_eq!(
        created["teacher_user_id"].as_i64(),
        Some(i64::from(teacher.id()))
    );

    let list_req = test::TestRequest::get()
        .uri("/reward-fraud-blocks?active=true&scope_type=teacher")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    let listed: Value = test::read_body_json(list_resp).await;
    assert!(listed
        .as_array()
        .expect("fraud block list")
        .iter()
        .any(|block| block["id"].as_i64() == Some(block_id)));

    let audit_req = test::TestRequest::get()
        .uri(&format!("/reward-fraud-blocks/{block_id}/audit"))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .to_request();
    let audit_resp = test::call_service(&app, audit_req).await;
    assert_eq!(audit_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(audit_resp).await;
    assert!(audit
        .as_array()
        .expect("fraud block audit")
        .iter()
        .any(|event| event["event_type"] == "created"
            && event["actor_user_id"].as_i64() == Some(i64::from(admin.id()))));

    let revoke_req = test::TestRequest::put()
        .uri(&format!("/reward-fraud-blocks/{block_id}/revoke"))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .to_request();
    let revoke_resp = test::call_service(&app, revoke_req).await;
    assert_eq!(revoke_resp.status(), StatusCode::OK);
    let revoked: Value = test::read_body_json(revoke_resp).await;
    assert_eq!(
        revoked["revoked_by_user_id"].as_i64(),
        Some(i64::from(admin.id()))
    );

    let audit_req = test::TestRequest::get()
        .uri(&format!("/reward-fraud-blocks/{block_id}/audit"))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .to_request();
    let audit_resp = test::call_service(&app, audit_req).await;
    assert_eq!(audit_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(audit_resp).await;
    assert!(audit
        .as_array()
        .expect("fraud block audit")
        .iter()
        .any(|event| event["event_type"] == "revoked"
            && event["actor_user_id"].as_i64() == Some(i64::from(admin.id()))));
}

#[actix_web::test]
async fn delegated_permission_api_grants_lists_and_revokes_reward_permissions() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "delegate_api_admin").await;
    let moderator = create_test_user(&mut conn, "delegate_api_moderator").await;
    let grantee = create_test_user(&mut conn, "delegate_api_grantee").await;
    assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, moderator.id(), "MODERATOR").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::delegated_permissions::delegated_permission_scope()),
    )
    .await;

    let grant_body = json!({
        "grantee_user_id": grantee.id(),
        "permission": Permissions::APPROVE_REWARD_AMOUNT.to_string(),
        "scope_type": "platform",
        "reason": "temporary reward amount review"
    });

    let denied_req = test::TestRequest::post()
        .uri("/delegated-permissions")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .set_json(&grant_body)
        .to_request();
    let denied_resp = test::call_service(&app, denied_req).await;
    assert_eq!(denied_resp.status(), StatusCode::FORBIDDEN);

    let grant_req = test::TestRequest::post()
        .uri("/delegated-permissions")
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .set_json(&grant_body)
        .to_request();
    let grant_resp = test::call_service(&app, grant_req).await;
    assert_eq!(grant_resp.status(), StatusCode::CREATED);
    let granted: Value = test::read_body_json(grant_resp).await;
    let delegation_id = granted["id"].as_i64().expect("delegation id");
    assert_eq!(
        granted["grantee_user_id"].as_i64(),
        Some(i64::from(grantee.id()))
    );
    assert_eq!(
        granted["permission"],
        Permissions::APPROVE_REWARD_AMOUNT.to_string()
    );
    assert_eq!(granted["scope_type"], "platform");

    let list_req = test::TestRequest::get()
        .uri(&format!(
            "/delegated-permissions?active=true&grantee_user_id={}",
            grantee.id()
        ))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    let listed: Value = test::read_body_json(list_resp).await;
    assert!(listed
        .as_array()
        .expect("delegation list")
        .iter()
        .any(|delegation| delegation["id"].as_i64() == Some(delegation_id)));

    let revoke_req = test::TestRequest::put()
        .uri(&format!("/delegated-permissions/{delegation_id}/revoke"))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .set_json(json!({ "revoke_reason": "assignment rotated" }))
        .to_request();
    let revoke_resp = test::call_service(&app, revoke_req).await;
    assert_eq!(revoke_resp.status(), StatusCode::OK);
    let revoked: Value = test::read_body_json(revoke_resp).await;
    assert_eq!(
        revoked["revoked_by_user_id"].as_i64(),
        Some(i64::from(admin.id()))
    );
    assert_eq!(revoked["revoke_reason"], "assignment rotated");
}
