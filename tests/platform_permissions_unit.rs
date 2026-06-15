use chrono::NaiveDate;
use rust_learn::domain::access_control::permissions::Permissions;
use rust_learn::domain::access_control::roles::Roles;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::infra::postgres::access_control::role_assignments::assign_platform_role_to_user;
use rust_learn::infra::postgres::establish_connection;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
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

    assign_platform_role_to_user(&mut conn, user.id(), Roles::SUPER_ADMIN)
        .await
        .expect("failed to assign SUPER_ADMIN role");

    // Test the platform role permission record lookup directly.
    let has_perm = platform_role_records::platform_user_has_permission(
        &mut conn,
        user.id(),
        &Permissions::MANAGE_PLATFORM_SETTINGS.to_string(),
    )
    .await
    .expect("query failed");

    assert!(
        has_perm,
        "SUPER_ADMIN should have MANAGE_PLATFORM_SETTINGS (direct call)"
    );
}
