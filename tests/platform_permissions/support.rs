pub(crate) use chrono::NaiveDate;
pub(crate) use rust_learn::domain::access_control::permissions::Permissions;
pub(crate) use rust_learn::domain::access_control::roles::Roles;
pub(crate) use rust_learn::infra::postgres::access_control::permission_queries::has_platform_permission;
pub(crate) use rust_learn::infra::postgres::access_control::role_assignments::assign_platform_permission_to_role;
pub(crate) use rust_learn::infra::postgres::access_control::role_assignments::assign_platform_role_to_user;
pub(crate) use rust_learn::infra::postgres::establish_connection;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;

pub(crate) fn unique_email(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}+{}@example.com", prefix, ts)
}

pub(crate) async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    // Load .env so DATABASE_URL and other envs are available in tests
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

#[actix_web::test]
pub(crate) async fn platform_super_admin_has_key_permissions() {
    let mut conn = setup_conn().await;

    // Create a fresh user and assign SUPER_ADMIN
    let email = unique_email("superadmin");
    let user = create_user(
        &mut conn,
        "Super Admin Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");

    assign_platform_role_to_user(&mut conn, user.id(), Roles::SUPER_ADMIN)
        .await
        .expect("failed to assign SUPER_ADMIN role");

    // Check a representative set of permissions that SUPER_ADMIN should have
    let perms = [
        Permissions::MANAGE_PLATFORM_SETTINGS,
        Permissions::VIEW_AUDIT_LOGS,
        Permissions::MANAGE_ROLE_PERMISSIONS,
        Permissions::RUN_TESTS,
        Permissions::MANAGE_SMART_CONTRACTS,
        Permissions::MANAGE_S3_OBJECTS,
        Permissions::VIEW_ANALYTICS_DASHBOARD,
    ];

    for p in perms {
        let ok = has_platform_permission(&mut conn, user.id(), &p.to_string())
            .await
            .expect("permission query failed");
        assert!(ok, "SUPER_ADMIN missing {:?}", p);
    }
}

#[actix_web::test]
pub(crate) async fn platform_admin_has_curated_permissions_but_not_all() {
    let mut conn = setup_conn().await;

    // Create a fresh user and assign ADMIN
    let email = unique_email("admin");
    let user = create_user(
        &mut conn,
        "Admin Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1992, 2, 2).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");

    assign_platform_role_to_user(&mut conn, user.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    // Positive cases (seeded for ADMIN)
    let allowed = [
        Permissions::MANAGE_PLATFORM_SETTINGS,
        Permissions::VIEW_AUDIT_LOGS,
        Permissions::MANAGE_ROLE_PERMISSIONS,
        Permissions::VIEW_NOTIFICATION,
        Permissions::VIEW_REPORT,
    ];

    for p in allowed {
        let ok = has_platform_permission(&mut conn, user.id(), &p.to_string())
            .await
            .expect("permission query failed");
        assert!(ok, "ADMIN should have {:?}", p);
    }

    // Negative cases (not granted to ADMIN in seed)
    let denied = [
        Permissions::IMPERSONATE_USER,       // explicitly excluded
        Permissions::MANAGE_SMART_CONTRACTS, // not in ADMIN seed list
    ];

    for p in denied {
        let ok = has_platform_permission(&mut conn, user.id(), &p.to_string())
            .await
            .expect("permission query failed");
        assert!(!ok, "ADMIN should NOT have {:?}", p);
    }
}
