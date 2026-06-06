use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::schema::{
    courses, external_transactions, internal_transactions, organization_roles, organizations,
    platform_roles, reward_candidates, reward_payout_records, reward_wallet_credit_records,
    role_permission_organization, role_permission_platform, transactions,
    transactions_external_transactions, transactions_internal_transactions,
    wallet_token_deposit_intents, wallets,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::course::NewCourse;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_WALLET_CREDITED,
};
use rust_learn::models::role::{OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::persistent_state_repository::set_persistent_state;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::wallet_service::{
    self, credit_observed_wallet_deposit, ObservedWalletDepositEvent, WalletTokenTransferRequest,
};
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::json;
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

async fn create_test_course(conn: &mut AsyncPgConnection) -> i32 {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string("wallet_audit_course"),
        })
        .returning(courses::id)
        .get_result(conn)
        .await
        .expect("failed to create wallet audit course")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_platform_permission_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: Permissions,
) {
    let role_id: i32 = diesel::insert_into(platform_roles::table)
        .values((
            platform_roles::name.eq(unique_string("wallet_platform_permission")),
            platform_roles::description.eq(Some("wallet permission test role")),
        ))
        .returning(platform_roles::id)
        .get_result(conn)
        .await
        .expect("failed to create platform permission test role");

    diesel::insert_into(role_permission_platform::table)
        .values((
            role_permission_platform::platform_role_id.eq(Some(role_id)),
            role_permission_platform::permission.eq(permission.to_string()),
        ))
        .execute(conn)
        .await
        .expect("failed to assign platform permission to test role");

    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform permission test role");
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

async fn assign_organization_permission_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: Permissions,
) {
    let role_id: i32 = diesel::insert_into(organization_roles::table)
        .values((
            organization_roles::name.eq(unique_string("wallet_org_permission")),
            organization_roles::description.eq(Some("wallet organization permission test role")),
        ))
        .returning(organization_roles::id)
        .get_result(conn)
        .await
        .expect("failed to create organization permission test role");

    diesel::insert_into(role_permission_organization::table)
        .values((
            role_permission_organization::organization_id.eq(None::<i32>),
            role_permission_organization::organization_role_id.eq(Some(role_id)),
            role_permission_organization::permission.eq(permission.to_string()),
        ))
        .execute(conn)
        .await
        .expect("failed to assign organization permission to test role");

    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization permission test role");
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
    let auditor = create_test_user(&mut conn, "wallet_auditor").await;
    let target = create_test_user(&mut conn, "wallet_target").await;
    let second_target = create_test_user(&mut conn, "wallet_target_two").await;
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

#[actix_web::test]
async fn organization_wallet_manager_can_link_and_read_org_wallet() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let org_admin = create_test_user(&mut conn, "wallet_org_admin").await;
    let org_moderator = create_test_user(&mut conn, "wallet_org_moderator").await;
    let org_reporter = create_test_user(&mut conn, "wallet_org_reporter").await;
    let stranger = create_test_user(&mut conn, "wallet_org_stranger").await;
    let org = create_test_organization(&mut conn).await;
    assign_organization_role(&mut conn, org_admin.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, org_moderator.id(), org.id, "MODERATOR").await;
    assign_organization_permission_role(
        &mut conn,
        org_reporter.id(),
        org.id,
        Permissions::VIEW_ORG_REWARD_REPORTS,
    )
    .await;
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

    let moderator_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_moderator.id())),
        ))
        .to_request();
    let moderator_resp = test::call_service(&app, moderator_req).await;
    assert_eq!(moderator_resp.status(), StatusCode::FORBIDDEN);

    let reporter_link_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_reporter.id())),
        ))
        .to_request();
    let reporter_link_resp = test::call_service(&app, reporter_link_req).await;
    assert_eq!(reporter_link_resp.status(), StatusCode::FORBIDDEN);

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

    let reporter_get_req = test::TestRequest::get()
        .uri(&format!("/api/wallets/organizations/{}", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_reporter.id())),
        ))
        .to_request();
    let reporter_get_resp = test::call_service(&app, reporter_get_req).await;
    assert_eq!(reporter_get_resp.status(), StatusCode::OK);
    let reporter_fetched: Value = test::read_body_json(reporter_get_resp).await;
    assert_eq!(reporter_fetched["id"], wallet_id);

    let moderator_get_req = test::TestRequest::get()
        .uri(&format!("/api/wallets/organizations/{}", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_moderator.id())),
        ))
        .to_request();
    let moderator_get_resp = test::call_service(&app, moderator_get_req).await;
    assert_eq!(moderator_get_resp.status(), StatusCode::FORBIDDEN);

    let mut conn = setup_conn(&pool).await;
    let count: i64 = wallets::table
        .filter(wallets::organization_id.eq(org.id))
        .count()
        .get_result(&mut conn)
        .await
        .expect("wallet count query should succeed");
    assert_eq!(count, 1);
}

#[actix_web::test]
async fn duplicate_pending_deposit_intents_are_marked_ambiguous_without_crediting() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let learner = create_test_user(&mut conn, "wallet_ambiguous_learner").await;

    set_persistent_state(
        &mut conn,
        "platform_importer_address",
        "0x00000000000000000000000000000000000000bb",
    )
    .await
    .expect("failed to configure platform importer address");

    let request = WalletTokenTransferRequest {
        amount: BigDecimal::from(20),
        ethereum_address: "0x00000000000000000000000000000000000000aa".to_string(),
        gas_payer: "platform".to_string(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: Some("0x00000000000000000000000000000000000000bb".to_string()),
    };

    let first =
        wallet_service::deposit_tokens_to_user_wallet(&mut conn, learner.id(), request.clone())
            .await
            .expect("first deposit intent should be created");
    let second = wallet_service::deposit_tokens_to_user_wallet(&mut conn, learner.id(), request)
        .await
        .expect("second deposit intent should be created");
    assert_ne!(first.id, second.id);
    assert_eq!(first.wallet_id, second.wallet_id);

    let tx_hash = unique_string("ambiguous_deposit_tx");
    let result = credit_observed_wallet_deposit(
        &mut conn,
        ObservedWalletDepositEvent {
            chain_id: 31337,
            contract_address: "0x00000000000000000000000000000000000000cc".to_string(),
            transaction_hash: tx_hash.clone(),
            log_index: 7,
            event_type: "import".to_string(),
            from_address: "0x00000000000000000000000000000000000000aa".to_string(),
            to_address: "0x00000000000000000000000000000000000000bb".to_string(),
            amount: BigDecimal::from(20),
        },
    )
    .await
    .expect("ambiguous observed deposit should be handled");
    assert!(!result.credited);
    assert_eq!(result.status, "ambiguous");

    let intent_rows = wallet_token_deposit_intents::table
        .filter(wallet_token_deposit_intents::id.eq_any([first.id, second.id]))
        .select((
            wallet_token_deposit_intents::status,
            wallet_token_deposit_intents::transaction_hash,
            wallet_token_deposit_intents::chain_id,
            wallet_token_deposit_intents::log_index,
        ))
        .load::<(String, Option<String>, Option<i64>, Option<i64>)>(&mut conn)
        .await
        .expect("deposit intents should be queryable");
    assert_eq!(intent_rows.len(), 2);
    for (status, stored_hash, chain_id, log_index) in intent_rows {
        assert_eq!(status, "ambiguous");
        assert_eq!(
            stored_hash.as_deref(),
            Some(tx_hash.to_ascii_lowercase().as_str())
        );
        assert_eq!(chain_id, Some(31337));
        assert_eq!(log_index, Some(7));
    }

    let wallet_balance: BigDecimal = wallets::table
        .find(first.wallet_id)
        .select(wallets::value)
        .get_result(&mut conn)
        .await
        .expect("wallet balance should be queryable");
    assert_eq!(wallet_balance, BigDecimal::from(0));
}

#[actix_web::test]
async fn wallet_token_deposit_and_retire_apply_platform_paid_tax() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let tax_admin = create_test_user(&mut conn, "wallet_tax_admin").await;
    let deposit_tax_only = create_test_user(&mut conn, "wallet_deposit_tax_only").await;
    let stranger = create_test_user(&mut conn, "wallet_tax_stranger").await;
    let learner = create_test_user(&mut conn, "wallet_tax_learner").await;
    assign_platform_permission_role(&mut conn, tax_admin.id(), Permissions::SET_DEPOSIT_TAX).await;
    assign_platform_permission_role(&mut conn, tax_admin.id(), Permissions::SET_RETIRE_TAX).await;
    assign_platform_permission_role(
        &mut conn,
        deposit_tax_only.id(),
        Permissions::SET_DEPOSIT_TAX,
    )
    .await;
    set_persistent_state(
        &mut conn,
        "platform_importer_address",
        "0x00000000000000000000000000000000000000bb",
    )
    .await
    .expect("failed to configure platform importer address");
    drop(conn);

    let app = test::init_service(wallet_test_app(pool.clone())).await;

    let forbidden_deposit_tax_req = test::TestRequest::put()
        .uri("/api/wallets/token-taxes/deposit")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .set_json(json!({ "tax_amount": "2" }))
        .to_request();
    let forbidden_deposit_tax_resp = test::call_service(&app, forbidden_deposit_tax_req).await;
    assert_eq!(forbidden_deposit_tax_resp.status(), StatusCode::FORBIDDEN);

    let forbidden_retire_tax_req = test::TestRequest::put()
        .uri("/api/wallets/token-taxes/retire")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(deposit_tax_only.id())),
        ))
        .set_json(json!({ "tax_amount": "1" }))
        .to_request();
    let forbidden_retire_tax_resp = test::call_service(&app, forbidden_retire_tax_req).await;
    assert_eq!(forbidden_retire_tax_resp.status(), StatusCode::FORBIDDEN);

    let set_deposit_tax_req = test::TestRequest::put()
        .uri("/api/wallets/token-taxes/deposit")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(tax_admin.id())),
        ))
        .set_json(json!({ "tax_amount": "2" }))
        .to_request();
    let set_deposit_tax_resp = test::call_service(&app, set_deposit_tax_req).await;
    assert_eq!(set_deposit_tax_resp.status(), StatusCode::OK);
    let deposit_tax: Value = test::read_body_json(set_deposit_tax_resp).await;
    assert_eq!(deposit_tax["operation"], "deposit");
    assert_eq!(deposit_tax["tax_amount"], "2");

    let set_retire_tax_req = test::TestRequest::put()
        .uri("/api/wallets/token-taxes/retire")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(tax_admin.id())),
        ))
        .set_json(json!({ "tax_amount": "1" }))
        .to_request();
    let set_retire_tax_resp = test::call_service(&app, set_retire_tax_req).await;
    assert_eq!(set_retire_tax_resp.status(), StatusCode::OK);
    let retire_tax: Value = test::read_body_json(set_retire_tax_resp).await;
    assert_eq!(retire_tax["operation"], "retire");
    assert_eq!(retire_tax["tax_amount"], "1");

    let taxes_req = test::TestRequest::get()
        .uri("/api/wallets/token-taxes")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let taxes_resp = test::call_service(&app, taxes_req).await;
    assert_eq!(taxes_resp.status(), StatusCode::OK);
    let taxes: Value = test::read_body_json(taxes_resp).await;
    assert_eq!(taxes["deposit"]["tax_amount"], "2");
    assert_eq!(taxes["retire"]["tax_amount"], "1");

    let deposit_tx_hash = format!(
        "0x{:064x}",
        ((std::process::id() as u128) << 64)
            | chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u128
    );
    let deposit_req = test::TestRequest::post()
        .uri("/api/wallets/me/deposits")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .set_json(json!({
            "amount": "20",
            "ethereum_address": "0x00000000000000000000000000000000000000aa",
            "platform_address": "0x00000000000000000000000000000000000000bb",
            "gas_payer": "platform",
            "chain_id": 31337,
            "contract_address": "0x00000000000000000000000000000000000000cc",
            "transaction_hash": deposit_tx_hash.clone(),
            "log_index": 0
        }))
        .to_request();
    let deposit_resp = test::call_service(&app, deposit_req).await;
    assert_eq!(deposit_resp.status(), StatusCode::CREATED);
    let deposit: Value = test::read_body_json(deposit_resp).await;
    assert_eq!(deposit["operation"], "deposit");
    assert_eq!(deposit["status"], "pending_chain_confirmation");
    assert_eq!(deposit["amount"], "20");
    assert_eq!(deposit["tax_amount"], "2");
    assert_eq!(deposit["wallet_delta_on_confirmation"], "18");
    assert_eq!(deposit["gas_payer"], "platform");
    assert_eq!(deposit["wallet_provider"], "metamask");
    assert_eq!(deposit["metamask_required"], true);
    assert_eq!(deposit["wallet_action"], "metamask_permit_signature");
    let deposit_intent_id = deposit["id"].as_i64().expect("deposit intent id");
    let wallet_id = deposit["wallet_id"].as_i64().expect("wallet id") as i32;

    let mut conn = setup_conn(&pool).await;
    let pending_balance: BigDecimal = wallets::table
        .find(wallet_id)
        .select(wallets::value)
        .get_result(&mut conn)
        .await
        .expect("wallet balance query should succeed");
    assert_eq!(pending_balance, BigDecimal::from(0));

    let deposit_credit = credit_observed_wallet_deposit(
        &mut conn,
        ObservedWalletDepositEvent {
            chain_id: 31337,
            contract_address: "0x00000000000000000000000000000000000000cc".to_string(),
            transaction_hash: deposit_tx_hash.clone(),
            log_index: 0,
            event_type: "import".to_string(),
            from_address: "0x00000000000000000000000000000000000000aa".to_string(),
            to_address: "0x00000000000000000000000000000000000000bb".to_string(),
            amount: BigDecimal::from(20),
        },
    )
    .await
    .expect("observed deposit should credit");
    assert!(deposit_credit.credited);
    assert_eq!(deposit_credit.intent_id, Some(deposit_intent_id));
    assert_eq!(deposit_credit.wallet_id, Some(wallet_id));
    assert_eq!(deposit_credit.status, "credited");
    let deposit_transaction_id = deposit_credit
        .transaction_id
        .expect("deposit transaction id");
    drop(conn);

    let retire_req = test::TestRequest::post()
        .uri("/api/wallets/me/retirements")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .set_json(json!({
            "amount": "5",
            "ethereum_address": "0x00000000000000000000000000000000000000dd",
            "platform_address": "0x00000000000000000000000000000000000000bb",
            "gas_payer": "platform",
            "chain_id": 31337,
            "contract_address": "0x00000000000000000000000000000000000000cc",
            "transaction_hash": unique_string("retire_tx"),
            "log_index": 1
        }))
        .to_request();
    let retire_resp = test::call_service(&app, retire_req).await;
    assert_eq!(retire_resp.status(), StatusCode::CREATED);
    let retire: Value = test::read_body_json(retire_resp).await;
    assert_eq!(retire["operation"], "retire");
    assert_eq!(retire["amount"], "5");
    assert_eq!(retire["tax_amount"], "1");
    assert_eq!(retire["wallet_delta"], "-6");
    assert_eq!(retire["gas_payer"], "platform");
    assert_eq!(retire["wallet_provider"], "platform");
    assert_eq!(retire["metamask_required"], false);
    assert_eq!(retire["wallet_action"], "platform_transfer");
    let retire_transaction_id = retire["transaction_id"]
        .as_i64()
        .expect("retire transaction id");

    let wallet_req = test::TestRequest::get()
        .uri("/api/wallets/me")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let wallet_resp = test::call_service(&app, wallet_req).await;
    assert_eq!(wallet_resp.status(), StatusCode::OK);
    let wallet: Value = test::read_body_json(wallet_resp).await;
    assert_eq!(wallet["id"], wallet_id);
    assert_eq!(wallet["value"], "12");

    let audit_req = test::TestRequest::get()
        .uri("/api/wallets/me/audit")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let audit_resp = test::call_service(&app, audit_req).await;
    assert_eq!(audit_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(audit_resp).await;

    let internal = audit["internal_transactions"]
        .as_array()
        .expect("internal transaction audit rows");
    assert_eq!(internal.len(), 4);
    assert!(internal.iter().any(|row| {
        row["transaction_id"] == deposit_transaction_id
            && row["transaction_type"] == "token_deposit"
            && row["amount"] == "20"
    }));
    assert!(internal.iter().any(|row| {
        row["transaction_id"] == deposit_transaction_id
            && row["transaction_type"] == "token_deposit"
            && row["amount"] == "-2"
    }));
    assert!(internal.iter().any(|row| {
        row["transaction_id"] == retire_transaction_id
            && row["transaction_type"] == "token_retire"
            && row["amount"] == "-5"
    }));
    assert!(internal.iter().any(|row| {
        row["transaction_id"] == retire_transaction_id
            && row["transaction_type"] == "token_retire"
            && row["amount"] == "-1"
    }));

    let external = audit["external_transactions"]
        .as_array()
        .expect("external transaction audit rows");
    assert_eq!(external.len(), 2);
    assert!(external.iter().any(|row| {
        row["transaction_id"] == deposit_transaction_id
            && row["event_type"] == "import"
            && row["reward_candidate_id"].is_null()
    }));
    assert!(external.iter().any(|row| {
        row["transaction_id"] == retire_transaction_id
            && row["event_type"] == "transfer"
            && row["reward_candidate_id"].is_null()
    }));

    let mut conn = setup_conn(&pool).await;
    let balance: BigDecimal = wallets::table
        .find(wallet_id)
        .select(wallets::value)
        .get_result(&mut conn)
        .await
        .expect("wallet balance query should succeed");
    assert_eq!(balance, BigDecimal::from(12));
}

#[actix_web::test]
async fn wallet_audit_view_includes_reward_transactions_and_reconciliation_status() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let student = create_test_user(&mut conn, "wallet_audit_student").await;
    let submitter = create_test_user(&mut conn, "wallet_audit_submitter").await;
    let course_id = create_test_course(&mut conn).await;
    let wallet = wallet_service::link_user_wallet(&mut conn, student.id())
        .await
        .expect("student wallet should link")
        .wallet;

    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id: student.id(),
            submitter_user_id: submitter.id(),
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("wallet_audit_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: REWARD_STATUS_WALLET_CREDITED.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create reward candidate for wallet audit");

    let amount = BigDecimal::from(12);
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::approved_amount.eq(Some(amount.clone())),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter.id())),
            reward_candidates::amount_decided_at.eq(Some(Utc::now())),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to mark reward candidate amount");

    let payout_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_payout"))
        .returning(transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create payout transaction");
    let external_transaction_id: i64 = diesel::insert_into(external_transactions::table)
        .values((
            external_transactions::amount.eq(amount.clone()),
            external_transactions::blockchain_address.eq("0xstudent"),
            external_transactions::chain_id.eq(Some(31337_i64)),
            external_transactions::contract_address.eq(Some("0xcontract")),
            external_transactions::transaction_hash.eq(Some(unique_string("wallet_audit_tx"))),
            external_transactions::log_index.eq(Some(0_i64)),
            external_transactions::event_type.eq(Some("Transfer")),
            external_transactions::from_address.eq(Some("0xtreasury")),
            external_transactions::to_address.eq(Some("0xstudent")),
        ))
        .returning(external_transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create external transaction");
    diesel::insert_into(transactions_external_transactions::table)
        .values((
            transactions_external_transactions::transaction_id.eq(payout_transaction_id),
            transactions_external_transactions::external_transaction_id.eq(external_transaction_id),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to link external transaction");
    let payout_record_id: i64 = diesel::insert_into(reward_payout_records::table)
        .values((
            reward_payout_records::reward_candidate_id.eq(candidate_id),
            reward_payout_records::transaction_id.eq(payout_transaction_id),
            reward_payout_records::external_transaction_id.eq(external_transaction_id),
        ))
        .returning(reward_payout_records::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create reward payout record");

    let wallet_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_wallet_credit"))
        .returning(transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create wallet credit transaction");
    let internal_transaction_id: i64 = diesel::insert_into(internal_transactions::table)
        .values((
            internal_transactions::wallet_id.eq(wallet.id),
            internal_transactions::amount.eq(amount.clone()),
        ))
        .returning(internal_transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create internal transaction");
    diesel::insert_into(transactions_internal_transactions::table)
        .values((
            transactions_internal_transactions::transaction_id.eq(wallet_transaction_id),
            transactions_internal_transactions::internal_transaction_id.eq(internal_transaction_id),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to link internal transaction");
    let credit_record_id: i64 = diesel::insert_into(reward_wallet_credit_records::table)
        .values((
            reward_wallet_credit_records::reward_candidate_id.eq(candidate_id),
            reward_wallet_credit_records::wallet_id.eq(wallet.id),
            reward_wallet_credit_records::transaction_id.eq(wallet_transaction_id),
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transaction_id),
        ))
        .returning(reward_wallet_credit_records::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create reward wallet credit record");
    diesel::update(wallets::table.find(wallet.id))
        .set(wallets::value.eq(amount.clone()))
        .execute(&mut conn)
        .await
        .expect("failed to update wallet balance");
    drop(conn);

    let app = test::init_service(wallet_test_app(pool.clone())).await;
    let audit_req = test::TestRequest::get()
        .uri("/api/wallets/me/audit")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let audit_resp = test::call_service(&app, audit_req).await;
    assert_eq!(audit_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(audit_resp).await;

    assert_eq!(audit["wallet"]["id"], wallet.id);
    assert_eq!(audit["wallet"]["owner_type"], "user");
    assert_eq!(audit["wallet"]["value"], "12");

    let internal = audit["internal_transactions"]
        .as_array()
        .expect("internal transaction audit rows");
    assert_eq!(internal.len(), 1);
    assert_eq!(
        internal[0]["internal_transaction_id"],
        internal_transaction_id
    );
    assert_eq!(internal[0]["transaction_id"], wallet_transaction_id);
    assert_eq!(internal[0]["transaction_type"], "reward_wallet_credit");
    assert_eq!(internal[0]["amount"], "12");

    let external = audit["external_transactions"]
        .as_array()
        .expect("external transaction audit rows");
    assert_eq!(external.len(), 1);
    assert_eq!(
        external[0]["external_transaction_id"],
        external_transaction_id
    );
    assert_eq!(external[0]["transaction_id"], payout_transaction_id);
    assert_eq!(external[0]["reward_candidate_id"], candidate_id);
    assert_eq!(external[0]["chain_id"], 31337);
    assert_eq!(external[0]["event_type"], "Transfer");

    let reward_records = audit["reward_records"]
        .as_array()
        .expect("reward audit rows");
    assert_eq!(reward_records.len(), 1);
    assert_eq!(reward_records[0]["reward_candidate_id"], candidate_id);
    assert_eq!(
        reward_records[0]["candidate_status"],
        REWARD_STATUS_WALLET_CREDITED
    );
    assert_eq!(
        reward_records[0]["reconciliation_status"],
        "needs_notification"
    );
    assert_eq!(
        reward_records[0]["wallet_credit_record_id"],
        credit_record_id
    );
    assert_eq!(
        reward_records[0]["wallet_credit_transaction_id"],
        wallet_transaction_id
    );
    assert_eq!(
        reward_records[0]["internal_transaction_id"],
        internal_transaction_id
    );
    assert_eq!(reward_records[0]["payout_record_id"], payout_record_id);
    assert_eq!(
        reward_records[0]["payout_transaction_id"],
        payout_transaction_id
    );
    assert_eq!(
        reward_records[0]["external_transaction_id"],
        external_transaction_id
    );
    assert!(reward_records[0]["notification_id"].is_null());
}
