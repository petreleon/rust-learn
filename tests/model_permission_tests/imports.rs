use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::users;
use rust_learn::infra::postgres::access_control::hierarchy_records;
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::course_role_records;
use rust_learn::infra::postgres::access_control::organization_role_records;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::repositories::user_repository::create_user;

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

async fn user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let u = create_user(
        conn,
        &format!("{} T", prefix),
        &(unique_string(prefix) + "@e.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("create user");
    diesel::update(users::table.find(u.id()))
        .set(users::email_verified.eq(true))
        .execute(conn)
        .await
        .unwrap();
    u
}

// ── Platform ──

#[actix_web::test]
async fn super_admin_has_all_permissions() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "perm_super").await;
    let role_id = role_catalog_store::platform_role_id_by_name(&mut conn, "SUPER_ADMIN")
        .await
        .unwrap();
    platform_role_records::assign_platform_role_to_user(&mut conn, u.id(), role_id)
        .await
        .unwrap();

    assert!(platform_role_records::platform_user_has_permission(
        &mut conn,
        u.id(),
        &Permissions::VIEW_REWARD_AUDIT.to_string(),
    )
    .await
    .unwrap());

    assert!(platform_role_records::platform_user_has_permission(
        &mut conn,
        u.id(),
        &Permissions::EXECUTE_REWARD_PAYOUT.to_string(),
    )
    .await
    .unwrap());
}

#[actix_web::test]
async fn regular_user_has_no_platform_permission() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "perm_nobody").await;

    assert!(!platform_role_records::platform_user_has_permission(
        &mut conn,
        u.id(),
        &Permissions::VIEW_REWARD_AUDIT.to_string(),
    )
    .await
    .unwrap());
}

#[actix_web::test]
async fn platform_hierarchy_super_admin_is_level_0() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "hier_super").await;
    let role_id = role_catalog_store::platform_role_id_by_name(&mut conn, "SUPER_ADMIN")
        .await
        .unwrap();
    platform_role_records::assign_platform_role_to_user(&mut conn, u.id(), role_id)
        .await
        .unwrap();

    let level = hierarchy_records::platform_min_level_for_user(&mut conn, u.id())
        .await
        .unwrap();
    assert_eq!(level, Some(0));
}

#[actix_web::test]
async fn unassigned_user_has_no_platform_level() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "hier_none").await;

    let level = hierarchy_records::platform_min_level_for_user(&mut conn, u.id())
        .await
        .unwrap();
    assert_eq!(level, None);
}

// ── Organization ──

#[actix_web::test]
async fn org_admin_has_org_permission() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "org_admin_p").await;
    let org_role_id = role_catalog_store::organization_role_id_by_name(&mut conn, "ADMIN")
        .await
        .unwrap();

    organization_role_records::assign_organization_role_to_user(&mut conn, u.id(), 1, org_role_id)
        .await
        .unwrap();

    assert!(organization_role_records::organization_user_has_permission(
        &mut conn,
        u.id(),
        1,
        &Permissions::VIEW_ORGANIZATION.to_string(),
    )
    .await
    .unwrap());
}

#[actix_web::test]
async fn org_stranger_has_no_org_permission() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "org_stranger").await;

    assert!(!organization_role_records::organization_user_has_permission(
        &mut conn,
        u.id(),
        999,
        &Permissions::VIEW_ORGANIZATION.to_string(),
    )
    .await
    .unwrap());
}
