use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rust_learn::db::establish_connection;
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::utils::db_utils::platform::{assign_role_to_user, user_permission_platform_request};
use rust_learn::config::constants::roles::Roles;
use rust_learn::config::constants::permissions::Permissions;
use chrono::NaiveDate;
use rust_learn::models::user_role_platform::UserRolePlatform;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos();
    format!("{}_{}", prefix, ts)
}

fn setup_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get().expect("failed to get DB connection from pool")
}

#[test]
fn test_platform_direct_has_permission_call() {
    let mut conn = setup_conn();

    let email = unique_string("platform_direct") + "@example.com";
    let user = create_user(
        &mut conn,
        "Platform Direct Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1995, 1, 1).unwrap()),
        "password123",
    )
    .expect("failed to create user");

    assign_role_to_user(&mut conn, user.id(), Roles::SUPER_ADMIN)
        .expect("failed to assign SUPER_ADMIN role");

    // Test calling UserRolePlatform::has_permission directly as requested
    let has_perm = UserRolePlatform::has_permission(&mut conn, user.id(), &Permissions::MANAGE_PLATFORM_SETTINGS.to_string())
        .expect("query failed");
    
    assert!(has_perm, "SUPER_ADMIN should have MANAGE_PLATFORM_SETTINGS (direct call)");
}
