use chrono::NaiveDate;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::establish_connection;
use rust_learn::repositories::platform_permission_repository::assign_permission_to_role_platform;
use rust_learn::repositories::platform_repository::{
    assign_role_to_user, user_permission_platform_request,
};
use rust_learn::repositories::user_repository::create_user;

fn unique_email(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}+{}@example.com", prefix, ts)
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    // Load .env so DATABASE_URL and other envs are available in tests
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

#[actix_web::test]
async fn platform_super_admin_has_key_permissions() {
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

    assign_role_to_user(&mut conn, user.id(), Roles::SUPER_ADMIN)
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
        let ok = user_permission_platform_request(&mut conn, user.id(), &p.to_string())
            .await
            .expect("permission query failed");
        assert!(ok, "SUPER_ADMIN missing {:?}", p);
    }
}

#[actix_web::test]
async fn platform_admin_has_curated_permissions_but_not_all() {
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

    assign_role_to_user(&mut conn, user.id(), Roles::ADMIN)
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
        let ok = user_permission_platform_request(&mut conn, user.id(), &p.to_string())
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
        let ok = user_permission_platform_request(&mut conn, user.id(), &p.to_string())
            .await
            .expect("permission query failed");
        assert!(!ok, "ADMIN should NOT have {:?}", p);
    }
}

#[actix_web::test]
async fn platform_admin_and_moderator_have_amount_approval_but_not_candidate_approval() {
    let mut conn = setup_conn().await;

    let admin = create_user(
        &mut conn,
        "Reward Admin Test",
        &unique_email("reward-admin"),
        Some(NaiveDate::from_ymd_opt(1994, 4, 4).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create admin user");
    let moderator = create_user(
        &mut conn,
        "Reward Moderator Test",
        &unique_email("reward-moderator"),
        Some(NaiveDate::from_ymd_opt(1995, 5, 5).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create moderator user");

    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    assign_role_to_user(&mut conn, moderator.id(), Roles::MODERATOR)
        .await
        .expect("failed to assign MODERATOR role");

    for user_id in [admin.id(), moderator.id()] {
        let can_approve_amount = user_permission_platform_request(
            &mut conn,
            user_id,
            &Permissions::APPROVE_REWARD_AMOUNT.to_string(),
        )
        .await
        .expect("permission query failed");
        assert!(
            can_approve_amount,
            "platform reviewer should approve amount"
        );

        let can_approve_candidate = user_permission_platform_request(
            &mut conn,
            user_id,
            &Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
        )
        .await
        .expect("permission query failed");
        assert!(
            !can_approve_candidate,
            "platform amount approval must not approve course reward candidates"
        );
    }

    let admin_can_block_teacher = user_permission_platform_request(
        &mut conn,
        admin.id(),
        &Permissions::BLOCK_REWARD_TEACHER.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        admin_can_block_teacher,
        "ADMIN should be able to block reward fraud"
    );

    let moderator_can_block_teacher = user_permission_platform_request(
        &mut conn,
        moderator.id(),
        &Permissions::BLOCK_REWARD_TEACHER.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        !moderator_can_block_teacher,
        "MODERATOR should not get fraud-block permission by default"
    );
}

#[actix_web::test]
async fn assign_permission_to_admin_and_verify_user_gets_it() {
    let mut conn = setup_conn().await;

    // Choose a permission that ADMIN does not have by default
    // Choose a permission that ADMIN does not have by default
    // Note: we'll reuse the enum value by referring to the constant again later to avoid move issues.
    // Assign it to ADMIN role (idempotent: insert or 0 rows if already exists)
    let rows =
        assign_permission_to_role_platform(&mut conn, Roles::ADMIN, Permissions::MANAGE_S3_OBJECTS)
            .await
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
    .await
    .expect("failed to create user");

    assign_role_to_user(&mut conn, user.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    // Now the permission should be granted to ADMIN users
    let ok = user_permission_platform_request(
        &mut conn,
        user.id(),
        &Permissions::MANAGE_S3_OBJECTS.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        ok,
        "ADMIN user should have MANAGE_S3_OBJECTS after assignment"
    );
}
