use crate::support::*;
use actix_web::{web, App};
use rust_learn::application::wallet::audit_wallet::WalletAuditUseCase;
use rust_learn::application::wallet::create_deposit_intent::WalletDepositIntentUseCase;
use rust_learn::application::wallet::link_wallet::WalletLinkUseCase;
use rust_learn::application::wallet::manage_token_tax::WalletTokenTaxUseCase;
use rust_learn::application::wallet::read_wallet::WalletReadUseCase;
use rust_learn::application::wallet::retire_tokens::WalletRetirementUseCase;
use rust_learn::infra::postgres::access_control::{
    organization_role_records, platform_role_records, role_catalog_store,
};
use rust_learn::infra::postgres::wallet::wallet_audit_use_case::PostgresWalletAuditUseCase;
use rust_learn::infra::postgres::wallet::wallet_deposit_intent_use_case::PostgresWalletDepositIntentUseCase;
use rust_learn::infra::postgres::wallet::wallet_link_use_case::PostgresWalletLinkUseCase;
use rust_learn::infra::postgres::wallet::wallet_read_use_case::PostgresWalletReadUseCase;
use rust_learn::infra::postgres::wallet::wallet_retirement_use_case::PostgresWalletRetirementUseCase;
use rust_learn::infra::postgres::wallet::wallet_token_tax_use_case::PostgresWalletTokenTaxUseCase;
use rust_learn::infra::tokens::jwt::create_jwt;
use std::sync::Arc;

pub(crate) async fn assign_platform_permission_role(
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

    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform permission test role");
}

pub(crate) async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name)
        .await
        .expect("organization role should exist");
    organization_role_records::assign_organization_role_to_user(
        conn,
        user_id,
        organization_id,
        role_id,
    )
    .await
    .expect("failed to assign organization role");
}

pub(crate) async fn assign_organization_permission_role(
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

    organization_role_records::assign_organization_role_to_user(
        conn,
        user_id,
        organization_id,
        role_id,
    )
    .await
    .expect("failed to assign organization permission test role");
}

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) fn wallet_test_app(
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
    let wallet_audit_use_case: Arc<dyn WalletAuditUseCase> =
        Arc::new(PostgresWalletAuditUseCase::new(pool.clone()));
    let wallet_deposit_intent_use_case: Arc<dyn WalletDepositIntentUseCase> =
        Arc::new(PostgresWalletDepositIntentUseCase::new(pool.clone()));
    let wallet_link_use_case: Arc<dyn WalletLinkUseCase> =
        Arc::new(PostgresWalletLinkUseCase::new(pool.clone()));
    let wallet_read_use_case: Arc<dyn WalletReadUseCase> =
        Arc::new(PostgresWalletReadUseCase::new(pool.clone()));
    let wallet_retirement_use_case: Arc<dyn WalletRetirementUseCase> =
        Arc::new(PostgresWalletRetirementUseCase::new(pool.clone()));
    let wallet_token_tax_use_case: Arc<dyn WalletTokenTaxUseCase> =
        Arc::new(PostgresWalletTokenTaxUseCase::new(pool.clone()));

    App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(wallet_audit_use_case))
        .app_data(web::Data::new(wallet_deposit_intent_use_case))
        .app_data(web::Data::new(wallet_link_use_case))
        .app_data(web::Data::new(wallet_read_use_case))
        .app_data(web::Data::new(wallet_retirement_use_case))
        .app_data(web::Data::new(wallet_token_tax_use_case))
        .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
        .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
        .service(web::scope("/api").configure(rust_learn::http::wallet::configure_routes))
}
