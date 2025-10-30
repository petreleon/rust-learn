use chrono::NaiveDate;
use rust_learn::db::establish_connection;
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::utils::db_utils::platform::{assign_role_to_user, user_permission_platform_request};
use rust_learn::utils::db_utils::platform_permission_utils::assign_permission_to_role_platform;
use rust_learn::config::constants::roles::Roles;
use rust_learn::config::constants::permissions::Permissions;

fn unique_email(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos();
    format!("{}+{}@example.com", prefix, ts)
}

fn setup_conn() -> diesel::PgConnection {
    // Load .env so DATABASE_URL and other envs are available in tests
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get().expect("failed to get DB connection from pool")
}

#[test]
fn platform_super_admin_has_key_permissions() {
    let mut conn = setup_conn();

    // Create a fresh user and assign SUPER_ADMIN
    let email = unique_email("superadmin");
    let user = create_user(
        &mut conn,
        "Super Admin Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .expect("failed to create user");

    assign_role_to_user(&mut conn, user.id(), Roles::SUPER_ADMIN)
        .expect("failed to assign SUPER_ADMIN role");

    // Check a representative set of permissions that SUPER_ADMIN should have
    let perms = [
        Permissions::MANAGE_PLATFORM_SETTINGS,
        Permissions::VIEW_AUDIT_LOGS,
        Permissions::MANAGE_ROLE_PERMISSIONS,
        Permissions::RUN_TESTS,
        Permissions::MANAGE_SMART_CONTRACTS,
        Permissions::MANAGE_MINIO_OBJECTS,
        Permissions::VIEW_ANALYTICS_DASHBOARD,
    ];

    for p in perms {
        let ok = user_permission_platform_request(&mut conn, user.id(), &p.to_string())
            .expect("permission query failed");
        assert!(ok, "SUPER_ADMIN missing {:?}", p);
    }
}

#[test]
fn platform_admin_has_curated_permissions_but_not_all() {
    let mut conn = setup_conn();

    // Create a fresh user and assign ADMIN
    let email = unique_email("admin");
    let user = create_user(
        &mut conn,
        "Admin Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1992, 2, 2).unwrap()),
        "password123",
    )
    .expect("failed to create user");

    assign_role_to_user(&mut conn, user.id(), Roles::ADMIN)
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
        let ok = user_permission_platform_request(&mut conn, user.id(), &p.to_string())
            .expect("permission query failed");
        assert!(ok, "ADMIN should have {:?}", p);
    }

    // Negative cases (not granted to ADMIN in seed)
    let denied = [
        Permissions::IMPERSONATE_USER,         // explicitly excluded
        Permissions::MANAGE_SMART_CONTRACTS,   // not in ADMIN seed list
    ];

    for p in denied {
        let ok = user_permission_platform_request(&mut conn, user.id(), &p.to_string())
            .expect("permission query failed");
        assert!(!ok, "ADMIN should NOT have {:?}", p);
    }
}

#[test]
fn assign_permission_to_admin_and_verify_user_gets_it() {
    let mut conn = setup_conn();

    // Choose a permission that ADMIN does not have by default
    let perm_to_add = Permissions::MANAGE_MINIO_OBJECTS;

    // Assign it to ADMIN role (idempotent: insert or 0 rows if already exists)
    let rows = assign_permission_to_role_platform(&mut conn, Roles::ADMIN, perm_to_add)
        .expect("failed to assign permission to ADMIN");
    assert!(rows == 0 || rows == 1, "unexpected rows affected: {}", rows);

    // Create a fresh user and assign ADMIN role
    let email = unique_email("admin-perm");
    let user = create_user(
        &mut conn,
        "Admin Perm Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1993, 3, 3).unwrap()),
        "password123",
    )
    .expect("failed to create user");

    assign_role_to_user(&mut conn, user.id(), Roles::ADMIN)
        .expect("failed to assign ADMIN role");

    // Now the permission should be granted to ADMIN users
    let ok = user_permission_platform_request(&mut conn, user.id(), &perm_to_add.to_string())
        .expect("permission query failed");
    assert!(ok, "ADMIN user should have {:?} after assignment", perm_to_add);
}
