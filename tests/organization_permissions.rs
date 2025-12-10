use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rust_learn::db::establish_connection;
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::utils::db_utils::organization::{assign_role_to_user_in_organization, user_permission_organization_request};
use rust_learn::config::constants::roles::Roles;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::db::schema::organizations;
use chrono::NaiveDate;
// We need to bypass the helper to setup the initial super-user/assigner
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::role::OrganizationRole;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos();
    format!("{}_{}", prefix, ts)
}

fn setup_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get().expect("failed to get DB connection from pool")
}

fn create_organization(conn: &mut PgConnection, name: &str) -> Organization {
    let new_org = NewOrganization {
        name: name.to_string(),
        website_link: None,
        profile_url: None,
    };

    diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result(conn)
        .expect("Error creating organization")
}

fn create_user_helper(conn: &mut PgConnection, prefix: &str) -> rust_learn::models::user::User {
    let email = unique_string(prefix) + "@example.com";
    create_user(
        conn,
        &format!("{} Test", prefix),
        &email,
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .expect("failed to create user")
}

// Low-level helper to setup the "God Mode" user for the tests
// We can't use assign_role_to_user_in_organization because it requires an assigner!
fn force_assign_role(conn: &mut PgConnection, user_id: i32, org_id: i32, role_name: &str) {
    let role_id = OrganizationRole::find_by_name(role_name, conn).expect("role not found");
    UserRoleOrganization::assign(conn, user_id, org_id, role_id).expect("force assign failed");
}

#[test]
fn org_admin_has_permissions() {
    let mut conn = setup_conn();
    let org_name = unique_string("OrgAdminTest");
    let org = create_organization(&mut conn, &org_name);
    
    let subject_user = create_user_helper(&mut conn, "subject_admin");
    // Force assign the role
    force_assign_role(&mut conn, subject_user.id(), org.id, "ADMIN");
    
    // Test Permissions
    let allowed_permissions = [
        Permissions::MANAGE_ORG_SETTINGS,
        Permissions::MANAGE_ORG_MEMBERS,
    ];

    for p in allowed_permissions {
        let has_perm = user_permission_organization_request(&mut conn, subject_user.id(), org.id, &p.to_string())
            .expect("permission query failed");
        assert!(has_perm, "ADMIN should have permission: {:?}", p);
    }
}

#[test]
fn org_member_has_limited_permissions() {
    let mut conn = setup_conn();
    let org_name = unique_string("OrgMemberTest");
    let org = create_organization(&mut conn, &org_name);

    let subject_user = create_user_helper(&mut conn, "subject_member");
    force_assign_role(&mut conn, subject_user.id(), org.id, "STUDENT");

    // Assuming STUDENT does not have MANAGE_ORG_SETTINGS
    let denied_permissions = [
        Permissions::MANAGE_ORG_SETTINGS,
    ];

    for p in denied_permissions {
        let has_perm = user_permission_organization_request(&mut conn, subject_user.id(), org.id, &p.to_string())
            .expect("permission query failed");
        assert!(!has_perm, "STUDENT should NOT have permission: {:?}", p);
    }
}

#[test]
fn assign_hierarchy_check_success() {
    let mut conn = setup_conn();
    let org_name = unique_string("HierSuccess");
    let org = create_organization(&mut conn, &org_name);

    // 1. Create an ADMIN (Assigner)
    let admin_user = create_user_helper(&mut conn, "assigner");
    force_assign_role(&mut conn, admin_user.id(), org.id, "ADMIN");

    // 2. Create a fresh user (Assignee)
    let member_user = create_user_helper(&mut conn, "assignee");

    // 3. Admin assigns STUDENT role to fresh user
    // Expect Success: Admin (1) is higher than Student role (4), and Admin (1) is higher than user (no role)
    let result = assign_role_to_user_in_organization(
        &mut conn, 
        admin_user.id(), 
        member_user.id(), 
        org.id, 
        "STUDENT"
    );
    assert!(result.is_ok(), "ADMIN should be able to assign STUDENT");
}

#[test]
fn assign_hierarchy_check_fail_assigning_higher_role() {
    let mut conn = setup_conn();
    let org_name = unique_string("HierFailRole");
    let org = create_organization(&mut conn, &org_name);

    // 1. Create a STUDENT (Assigner) - trying to punch up
    let member_user_assigner = create_user_helper(&mut conn, "assigner_weak");
    force_assign_role(&mut conn, member_user_assigner.id(), org.id, "STUDENT");

    // 2. Create a fresh user (Assignee)
    let new_user = create_user_helper(&mut conn, "assignee_new");

    // 3. Member tries to assign ADMIN
    // Expect Fail: Student (4) is NOT higher than Admin role (1)
    let result = assign_role_to_user_in_organization(
        &mut conn, 
        member_user_assigner.id(), 
        new_user.id(), 
        org.id, 
        "ADMIN"
    );
    assert!(result.is_err(), "STUDENT should NOT be able to assign ADMIN");
}

#[test]
fn assign_hierarchy_check_fail_assigning_to_higher_user() {
    let mut conn = setup_conn();
    let org_name = unique_string("HierFailUser");
    let org = create_organization(&mut conn, &org_name);

    // 1. Create a STUDENT (Assigner)
    let member_user_assigner = create_user_helper(&mut conn, "assigner_weak");
    force_assign_role(&mut conn, member_user_assigner.id(), org.id, "STUDENT");

    // 2. Create an ADMIN (Target User) - already has high status
    let admin_target = create_user_helper(&mut conn, "target_strong");
    force_assign_role(&mut conn, admin_target.id(), org.id, "ADMIN");

    // 3. Member tries to assign STUDENT role to ADMIN 
    // Expect Fail: Student (4) is NOT higher than Admin User (1)
    let result = assign_role_to_user_in_organization(
        &mut conn, 
        member_user_assigner.id(), 
        admin_target.id(), 
        org.id, 
        "STUDENT"
    );
    assert!(result.is_err(), "STUDENT should NOT be able to modify ADMIN");
}
