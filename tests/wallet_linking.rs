use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{organizations, wallets};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::{OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::Value;

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

async fn create_test_organization(conn: &mut AsyncPgConnection) -> Organization {
    let new_org = NewOrganization {
        name: unique_string("wallet_org"),
        website_link: None,
        profile_url: None,
    };

    diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role should exist");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn wallet_test_app(
    pool: DbPool,
) -> App<
    impl actix_service::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(pool))
        .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
        .service(web::scope("/api").service(rust_learn::api::wallets::wallet_scope()))
}

#[actix_web::test]
async fn user_can_link_and_read_own_wallet_idempotently() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let user = create_test_user(&mut conn, "wallet_self").await;
    drop(conn);

    let app = test::init_service(wallet_test_app(pool.clone())).await;
    let token = token_for(user.id());

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
    let target = create_test_user(&mut conn, "wallet_target").await;
    assign_platform_role(&mut conn, manager.id(), "ADMIN").await;
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
}

#[actix_web::test]
async fn organization_wallet_manager_can_link_and_read_org_wallet() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let org_admin = create_test_user(&mut conn, "wallet_org_admin").await;
    let stranger = create_test_user(&mut conn, "wallet_org_stranger").await;
    let org = create_test_organization(&mut conn).await;
    assign_organization_role(&mut conn, org_admin.id(), org.id, "ADMIN").await;
    drop(conn);

    let app = test::init_service(wallet_test_app(pool.clone())).await;

    let forbidden_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_resp = test::call_service(&app, forbidden_req).await;
    assert_eq!(forbidden_resp.status(), StatusCode::FORBIDDEN);

    let link_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let link_resp = test::call_service(&app, link_req).await;
    assert_eq!(link_resp.status(), StatusCode::CREATED);
    let first: Value = test::read_body_json(link_resp).await;
    assert_eq!(first["created"], true);
    assert_eq!(first["wallet"]["owner_type"], "organization");
    assert_eq!(first["wallet"]["organization_id"], org.id);
    assert!(first["wallet"]["user_id"].is_null());
    let wallet_id = first["wallet"]["id"].as_i64().expect("wallet id");

    let duplicate_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let duplicate_resp = test::call_service(&app, duplicate_req).await;
    assert_eq!(duplicate_resp.status(), StatusCode::OK);
    let duplicate: Value = test::read_body_json(duplicate_resp).await;
    assert_eq!(duplicate["created"], false);
    assert_eq!(duplicate["wallet"]["id"], wallet_id);

    let get_req = test::TestRequest::get()
        .uri(&format!("/api/wallets/organizations/{}", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let get_resp = test::call_service(&app, get_req).await;
    assert_eq!(get_resp.status(), StatusCode::OK);
    let fetched: Value = test::read_body_json(get_resp).await;
    assert_eq!(fetched["id"], wallet_id);
    assert_eq!(fetched["organization_id"], org.id);

    let mut conn = setup_conn(&pool).await;
    let count: i64 = wallets::table
        .filter(wallets::organization_id.eq(org.id))
        .count()
        .get_result(&mut conn)
        .await
        .expect("wallet count query should succeed");
    assert_eq!(count, 1);
}
