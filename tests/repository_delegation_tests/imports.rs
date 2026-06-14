use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::users;
use rust_learn::domain::access_control::delegation::DELEGATED_SCOPE_COURSE;
use rust_learn::models::delegated_permission::NewDelegatedPermission;
use rust_learn::models::user::User;
use rust_learn::repositories::delegated_permission_repository::{self, DelegatedPermissionFilter};
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

async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let user = create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");
    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(true))
        .execute(conn)
        .await
        .expect("failed to verify user email");
    user
}

fn new_del(grantor: i32, grantee: i32, permission: &str, course_id: i32) -> NewDelegatedPermission {
    NewDelegatedPermission {
        grantor_user_id: grantor,
        grantee_user_id: grantee,
        permission: permission.to_string(),
        scope_type: DELEGATED_SCOPE_COURSE.to_string(),
        organization_id: None,
        course_id: Some(course_id),
        reason: None,
        expires_at: None,
    }
}

#[actix_web::test]
async fn test_create_and_find() {
    let mut conn = setup_conn().await;
    let grantor = create_user_helper(&mut conn, "d1g").await;
    let grantee = create_user_helper(&mut conn, "d1e").await;

    let d = delegated_permission_repository::create_delegated_permission(
        &mut conn,
        new_del(grantor.id(), grantee.id(), "VIEW_COURSE_REWARD_STATUS", 1),
    )
    .await
    .unwrap();

    assert_eq!(d.grantor_user_id, grantor.id());
    assert_eq!(d.grantee_user_id, grantee.id());

    let found = delegated_permission_repository::find_delegated_permission(&mut conn, d.id)
        .await
        .unwrap();
    assert_eq!(found.id, d.id);
}

#[actix_web::test]
async fn test_duplicate_returns_existing() {
    let mut conn = setup_conn().await;
    let grantor = create_user_helper(&mut conn, "d2g").await;
    let grantee = create_user_helper(&mut conn, "d2e").await;

    let d1 = delegated_permission_repository::create_delegated_permission(
        &mut conn,
        new_del(grantor.id(), grantee.id(), "VIEW_COURSE_REWARD_STATUS", 1),
    )
    .await
    .unwrap();

    let d2 = delegated_permission_repository::create_delegated_permission(
        &mut conn,
        new_del(grantor.id(), grantee.id(), "VIEW_COURSE_REWARD_STATUS", 1),
    )
    .await
    .unwrap();

    assert_eq!(d1.id, d2.id);
}

#[actix_web::test]
async fn test_find_active() {
    let mut conn = setup_conn().await;
    let grantor = create_user_helper(&mut conn, "d3g").await;
    let grantee = create_user_helper(&mut conn, "d3e").await;

    delegated_permission_repository::create_delegated_permission(
        &mut conn,
        new_del(grantor.id(), grantee.id(), "VIEW_COURSE_REWARD_STATUS", 2),
    )
    .await
    .unwrap();

    let active = delegated_permission_repository::find_active_delegated_permission(
        &mut conn,
        grantee.id(),
        "VIEW_COURSE_REWARD_STATUS",
        DELEGATED_SCOPE_COURSE,
        None,
        Some(2),
    )
    .await
    .unwrap();
    assert!(active.is_some());

    let none = delegated_permission_repository::find_active_delegated_permission(
        &mut conn,
        grantee.id(),
        "VIEW_COURSE_REWARD_STATUS",
        DELEGATED_SCOPE_COURSE,
        None,
        Some(999),
    )
    .await
    .unwrap();
    assert!(none.is_none());
}

#[actix_web::test]
async fn test_revoke() {
    let mut conn = setup_conn().await;
    let grantor = create_user_helper(&mut conn, "d4g").await;
    let grantee = create_user_helper(&mut conn, "d4e").await;
    let revoker = create_user_helper(&mut conn, "d4r").await;

    let d = delegated_permission_repository::create_delegated_permission(
        &mut conn,
        new_del(grantor.id(), grantee.id(), "VIEW_COURSE_REWARD_STATUS", 3),
    )
    .await
    .unwrap();

    let revoked = delegated_permission_repository::revoke_delegated_permission(
        &mut conn,
        d.id,
        revoker.id(),
        Some("no longer needed".to_string()),
    )
    .await
    .unwrap();

    assert!(revoked.revoked_at.is_some());
    assert_eq!(revoked.revoked_by_user_id, Some(revoker.id()));
}
