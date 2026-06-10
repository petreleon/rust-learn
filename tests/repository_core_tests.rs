use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, organizations, users};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::course_repository::user_permission_course_request;
use rust_learn::repositories::organization_repository::{
    assign_role_to_user_in_organization, user_hierarchy_compare_organization,
    user_permission_organization_request,
};
use rust_learn::repositories::platform_repository::user_permission_platform_request;
use rust_learn::repositories::user_repository::create_user;
use std::cmp::Ordering;

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

async fn create_user_helper(
    conn: &mut AsyncPgConnection,
    prefix: &str,
    verified: bool,
) -> User {
    let user = create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");
    if verified {
        diesel::update(users::table.find(user.id()))
            .set(users::email_verified.eq(true))
            .execute(conn)
            .await
            .expect("failed to verify user email");
    }
    user
}

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: name.to_string(),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn get_org_admin_role_id(conn: &mut AsyncPgConnection) -> i32 {
    OrganizationRole::find_by_name("ADMIN", conn)
        .await
        .expect("organization admin role not found")
}

async fn get_org_member_role_id(conn: &mut AsyncPgConnection) -> i32 {
    OrganizationRole::find_by_name("STUDENT", conn)
        .await
        .expect("organization student role not found")
}

async fn get_course_admin_role_id(conn: &mut AsyncPgConnection) -> i32 {
    CourseRole::find_by_name("TEACHER", conn)
        .await
        .expect("course teacher role not found")
}

async fn get_course_student_role_id(conn: &mut AsyncPgConnection) -> i32 {
    CourseRole::find_by_name("STUDENT", conn)
        .await
        .expect("course student role not found")
}

async fn assign_org_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    org_id: i32,
    role_id: i32,
) {
    UserRoleOrganization::assign(conn, user_id, org_id, role_id)
        .await
        .expect("failed to assign org role");
}

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_id: i32,
) {
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

// ── user_repository ──

#[actix_web::test]
async fn test_create_user_with_verified_email() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "repo_user", true).await;
    assert_eq!(user.name, format!("repo_user Test"));
}

#[actix_web::test]
async fn test_create_user_with_unverified_email() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "repo_user_unver", false).await;
    assert_eq!(user.name, format!("repo_user_unver Test"));
}

// ── organization_repository ──

#[actix_web::test]
async fn test_org_permission_check_admin_has_admin_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "org_perm_admin", true).await;
    let org = create_organization(&mut conn, &unique_string("org_perm")).await;
    let admin_role_id = get_org_admin_role_id(&mut conn).await;
    assign_org_role(&mut conn, user.id(), org.id, admin_role_id).await;

    let has_permission = user_permission_organization_request(
        &mut conn,
        user.id(),
        org.id,
        &Permissions::VIEW_ORGANIZATION.to_string(),
    )
    .await
    .unwrap();
    assert!(has_permission);
}

#[actix_web::test]
async fn test_org_permission_check_stranger_has_no_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "org_perm_stranger", true).await;
    let org = create_organization(&mut conn, &unique_string("org_perm_stranger")).await;

    let has_permission = user_permission_organization_request(
        &mut conn,
        user.id(),
        org.id,
        &Permissions::VIEW_ORGANIZATION.to_string(),
    )
    .await
    .unwrap();
    assert!(!has_permission);
}

#[actix_web::test]
async fn test_org_hierarchy_admin_above_member() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "org_hier_admin", true).await;
    let member = create_user_helper(&mut conn, "org_hier_mem", true).await;
    let org = create_organization(&mut conn, &unique_string("org_hier")).await;
    let admin_role_id = get_org_admin_role_id(&mut conn).await;
    let member_role_id = get_org_member_role_id(&mut conn).await;
    assign_org_role(&mut conn, admin.id(), org.id, admin_role_id).await;
    assign_org_role(&mut conn, member.id(), org.id, member_role_id).await;

    let cmp = user_hierarchy_compare_organization(&mut conn, org.id, admin.id(), member.id())
        .await
        .unwrap();
    assert_eq!(cmp, Ordering::Greater);
}

#[actix_web::test]
async fn test_org_hierarchy_equal_users() {
    let mut conn = setup_conn().await;
    let user1 = create_user_helper(&mut conn, "org_hier_eq1", true).await;
    let user2 = create_user_helper(&mut conn, "org_hier_eq2", true).await;
    let org = create_organization(&mut conn, &unique_string("org_hier_eq")).await;
    let admin_role_id = get_org_admin_role_id(&mut conn).await;
    assign_org_role(&mut conn, user1.id(), org.id, admin_role_id).await;
    assign_org_role(&mut conn, user2.id(), org.id, admin_role_id).await;

    let cmp = user_hierarchy_compare_organization(&mut conn, org.id, user1.id(), user2.id())
        .await
        .unwrap();
    assert_eq!(cmp, Ordering::Equal);
}

#[actix_web::test]
async fn test_org_role_assignment_admin_can_assign_member() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "org_role_admin", true).await;
    let member = create_user_helper(&mut conn, "org_role_mem", true).await;
    let org = create_organization(&mut conn, &unique_string("org_role")).await;
    let admin_role_id = get_org_admin_role_id(&mut conn).await;
    assign_org_role(&mut conn, admin.id(), org.id, admin_role_id).await;

    let result = assign_role_to_user_in_organization(
        &mut conn,
        admin.id(),
        member.id(),
        org.id,
        "STUDENT",
    )
    .await;
    assert!(result.is_ok());
}

// ── course_repository ──

#[actix_web::test]
async fn test_course_permission_check_admin_has_admin_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "course_perm_admin", true).await;
    let course = create_course(&mut conn, &unique_string("course_perm")).await;
    let admin_role_id = get_course_admin_role_id(&mut conn).await;
    assign_course_role(&mut conn, user.id(), course.id, admin_role_id).await;

    let has_permission = user_permission_course_request(
        &mut conn,
        user.id(),
        course.id,
        &Permissions::MANAGE_COURSE_SETTINGS.to_string(),
    )
    .await
    .unwrap();
    assert!(has_permission);
}

#[actix_web::test]
async fn test_course_permission_check_student_has_no_admin_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "course_perm_student", true).await;
    let course = create_course(&mut conn, &unique_string("course_perm_student")).await;
    let student_role_id = get_course_student_role_id(&mut conn).await;
    assign_course_role(&mut conn, user.id(), course.id, student_role_id).await;

    let has_permission = user_permission_course_request(
        &mut conn,
        user.id(),
        course.id,
        &Permissions::MANAGE_COURSE_SETTINGS.to_string(),
    )
    .await
    .unwrap();
    assert!(!has_permission);
}

#[actix_web::test]
async fn test_course_role_assignment_admin_can_assign_student() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "course_role_admin", true).await;
    let student = create_user_helper(&mut conn, "course_role_stud", true).await;
    let course = create_course(&mut conn, &unique_string("course_role")).await;
    let admin_role_id = get_course_admin_role_id(&mut conn).await;
    assign_course_role(&mut conn, admin.id(), course.id, admin_role_id).await;

    let result = rust_learn::repositories::course_repository::assign_role_to_user_in_course(
        &mut conn,
        admin.id(),
        student.id(),
        course.id,
        "STUDENT",
    )
    .await;
    assert!(result.is_ok());
}

// ── platform_repository ──

#[actix_web::test]
async fn test_platform_permission_check_super_admin_has_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "plat_perm_admin", true).await;
    let role_id = PlatformRole::find_by_name("SUPER_ADMIN", &mut conn)
        .await
        .expect("super admin role not found");
    UserRolePlatform::assign(&mut conn, user.id(), role_id)
        .await
        .expect("failed to assign platform role");

    let has_permission = user_permission_platform_request(
        &mut conn,
        user.id(),
        &Permissions::VIEW_REWARD_AUDIT.to_string(),
    )
    .await
    .unwrap();
    assert!(has_permission);
}

#[actix_web::test]
async fn test_platform_permission_check_regular_user_has_no_perm() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "plat_perm_regular", true).await;

    let has_permission = user_permission_platform_request(
        &mut conn,
        user.id(),
        &Permissions::VIEW_REWARD_AUDIT.to_string(),
    )
    .await
    .unwrap();
    assert!(!has_permission);
}
