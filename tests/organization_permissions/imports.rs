use chrono::NaiveDate;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::organizations;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::repositories::organization_repository::{
    assign_role_to_user_in_organization, user_permission_organization_request,
};
use rust_learn::repositories::user_repository::create_user;
// We need to bypass the helper to setup the initial super-user/assigner
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::models::role::OrganizationRole;
use rust_learn::infra::postgres::access_control::organization_role_records;
use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let seq = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, seq)
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
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

async fn create_user_helper(
    conn: &mut AsyncPgConnection,
    prefix: &str,
) -> rust_learn::models::user::User {
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
// We can't use assign_role_to_user_in_organization because it requires an assigner!
async fn force_assign_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    org_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("role not found");
    organization_role_records::assign_organization_role_to_user(conn, user_id, org_id, role_id)
        .await
        .expect("force assign failed");
}

#[actix_web::test]
async fn org_admin_has_permissions() {
    let mut conn = setup_conn().await;
    let org_name = unique_string("OrgAdminTest");
    let org = create_organization(&mut conn, &org_name).await;

    let subject_user = create_user_helper(&mut conn, "subject_admin").await;
    // Force assign the role
    force_assign_role(&mut conn, subject_user.id(), org.id, "ADMIN").await;

    // Test Permissions
    let allowed_permissions = [
        Permissions::MANAGE_ORG_SETTINGS,
        Permissions::MANAGE_ORG_MEMBERS,
    ];

    for p in allowed_permissions {
        let has_perm = user_permission_organization_request(
            &mut conn,
            subject_user.id(),
            org.id,
            &p.to_string(),
        )
        .await
        .expect("permission query failed");
        assert!(has_perm, "ADMIN should have permission: {:?}", p);
    }
}
