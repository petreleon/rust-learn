use diesel::prelude::*;
use rust_learn::db::establish_connection;
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::utils::db_utils::platform::assign_role_to_user;
use rust_learn::config::constants::roles::Roles;
use rust_learn::config::constants::permissions::Permissions;
use chrono::NaiveDate;
use rust_learn::models::user_role_platform::UserRolePlatform;
use diesel_async::AsyncPgConnection;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn() -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get().await.expect("failed to get DB connection from pool")
}

#[actix_web::test]
async fn test_platform_direct_has_permission_call() {
    let mut conn = setup_conn().await;

    let email = unique_string("platform_direct") + "@example.com";
    let user = create_user(
        &mut conn,
        "Platform Direct Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1995, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");

    assign_role_to_user(&mut conn, user.id(), Roles::SUPER_ADMIN)
        .await
        .expect("failed to assign SUPER_ADMIN role");

    // Test calling UserRolePlatform::has_permission directly as requested
    let has_perm = UserRolePlatform::has_permission(&mut conn, user.id(), &Permissions::MANAGE_PLATFORM_SETTINGS.to_string())
        .await
        .expect("query failed");
    
    assert!(has_perm, "SUPER_ADMIN should have MANAGE_PLATFORM_SETTINGS (direct call)");
}
