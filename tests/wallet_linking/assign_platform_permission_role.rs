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
    let wallet_audit_use_case: Arc<dyn WalletAuditUseCase> =
        Arc::new(PostgresWalletAuditUseCase::new(pool.clone()));
    let wallet_read_use_case: Arc<dyn WalletReadUseCase> =
        Arc::new(PostgresWalletReadUseCase::new(pool.clone()));

    App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(wallet_audit_use_case))
        .app_data(web::Data::new(wallet_read_use_case))
        .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
        .service(web::scope("/api").service(rust_learn::api::wallets::wallet_scope()))
}
