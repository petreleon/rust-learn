pub(crate) use chrono::NaiveDate;
pub(crate) use rust_learn::domain::access_control::permissions::Permissions;
pub(crate) use rust_learn::infra::postgres::access_control::permission_queries::has_organization_permission;
pub(crate) use rust_learn::infra::postgres::access_control::role_assignments::assign_organization_role_with_hierarchy;
pub(crate) use rust_learn::infra::postgres::establish_connection;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::schema::organizations;
// We need to bypass the helper to setup the initial super-user/assigner
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let seq = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, seq)
}

pub(crate) async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    let new_org = NewOrganization {
        name: name.to_string(),
        website_link: None,
        profile_url: None,
    };

    diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result(conn)
        .await
        .expect("Error creating organization")
}

pub(crate) async fn create_user_helper(
    conn: &mut AsyncPgConnection,
    prefix: &str,
) -> rust_learn::infra::postgres::models::user::User {
    let email = unique_string(prefix) + "@example.com";
    create_user(
        conn,
        &format!("{} Test", prefix),
        &email,
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user")
}

// Low-level helper to setup the "God Mode" user for the tests
// We can't use assign_organization_role_with_hierarchy because it requires an assigner!
pub(crate) async fn force_assign_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    org_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name)
        .await
        .expect("role not found");
    organization_role_records::assign_organization_role_to_user(conn, user_id, org_id, role_id)
        .await
        .expect("force assign failed");
}
