use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::wallet::burn_tokens::{
    TokenBurnCommand, TokenBurnReconciliationCommand, TokenBurnSubject, TokenBurnUseCase,
};
use rust_learn::domain::access_control::permissions::Permissions;
use rust_learn::domain::wallet::burn::{
    TOKEN_BURN_STATUS_LEADERBOARD_INDEXED, TOKEN_BURN_STATUS_NEEDS_RECONCILIATION,
};
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::infra::postgres::establish_connection;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user;
use rust_learn::infra::postgres::schema::{
    platform_roles, role_permission_platform, token_burn_leaderboard_events, token_burn_requests,
    users,
};
use rust_learn::infra::postgres::wallet::wallet_burn_use_case::PostgresTokenBurnUseCase;
use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[actix_web::test]
async fn token_burn_reconciliation_indexes_leaderboard_once() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = pool.get().await.expect("db connection");
    let actor_id = create_reconcile_actor(&mut conn).await;
    let use_case = PostgresTokenBurnUseCase::new(pool);

    let burn = use_case
        .request_token_burn(actor_id, TokenBurnSubject::OwnUser, burn_command())
        .await
        .expect("burn request should be created");

    diesel::delete(
        token_burn_leaderboard_events::table
            .filter(token_burn_leaderboard_events::burn_request_id.eq(burn.id)),
    )
    .execute(&mut conn)
    .await
    .expect("leaderboard event should be removable");
    diesel::update(token_burn_requests::table.find(burn.id))
        .set((
            token_burn_requests::status.eq(TOKEN_BURN_STATUS_NEEDS_RECONCILIATION),
            token_burn_requests::leaderboard_indexed_at.eq(None::<DateTime<Utc>>),
        ))
        .execute(&mut conn)
        .await
        .expect("burn should be marked for reconciliation");

    let first = use_case
        .reconcile_token_burn(actor_id, burn.id, empty_reconciliation_command())
        .await
        .expect("first reconciliation should repair leaderboard event");
    let second = use_case
        .reconcile_token_burn(actor_id, burn.id, empty_reconciliation_command())
        .await
        .expect("second reconciliation should be idempotent");

    assert_eq!(first.status, TOKEN_BURN_STATUS_LEADERBOARD_INDEXED);
    assert_eq!(second.status, TOKEN_BURN_STATUS_LEADERBOARD_INDEXED);
    let event_count = token_burn_leaderboard_events::table
        .filter(token_burn_leaderboard_events::burn_request_id.eq(burn.id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("leaderboard events should be countable");
    assert_eq!(event_count, 1);
}

async fn create_reconcile_actor(conn: &mut AsyncPgConnection) -> i32 {
    let user = create_verified_password_user(
        conn,
        "Burn Reconciler",
        &(unique_string("burn_reconciler") + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("user should be created");
    diesel::update(users::table.find(user.id()))
        .set(users::kyc_verified.eq(true))
        .execute(conn)
        .await
        .expect("kyc should be enabled");
    assign_reconcile_permission(conn, user.id()).await;
    user.id()
}

async fn assign_reconcile_permission(conn: &mut AsyncPgConnection, user_id: i32) {
    let role_id = diesel::insert_into(platform_roles::table)
        .values((
            platform_roles::name.eq(unique_string("burn_reconciler_role")),
            platform_roles::description.eq(Some("token burn reconciliation test role")),
        ))
        .returning(platform_roles::id)
        .get_result::<i32>(conn)
        .await
        .expect("platform role should be created");
    diesel::insert_into(role_permission_platform::table)
        .values((
            role_permission_platform::platform_role_id.eq(Some(role_id)),
            role_permission_platform::permission.eq(Permissions::RECONCILE_TOKEN_BURNS.to_string()),
        ))
        .execute(conn)
        .await
        .expect("permission should be assigned");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("role should be assigned");
}

fn burn_command() -> TokenBurnCommand {
    TokenBurnCommand {
        amount: BigDecimal::from(9),
        source: "decentralized_direct".to_string(),
        fee_path: "network_fee_paid_by_user".to_string(),
        idempotency_key: unique_string("burn-key"),
        ethereum_address: Some("0x00000000000000000000000000000000000000ab".to_string()),
        platform_address: None,
        chain_id: Some(31337),
        contract_address: Some("0x00000000000000000000000000000000000000cd".to_string()),
        transaction_hash: Some(unique_string("0xtokenburn")),
        log_index: Some(0),
        deposit_intent_id: None,
        leaderboard_visible: Some(true),
    }
}

fn empty_reconciliation_command() -> TokenBurnReconciliationCommand {
    TokenBurnReconciliationCommand {
        ethereum_address: None,
        platform_address: None,
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        mark_failed: None,
        error_message: None,
    }
}

fn unique_string(prefix: &str) -> String {
    let seq = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!(
        "{}_{}_{}",
        prefix,
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
        seq
    )
}
