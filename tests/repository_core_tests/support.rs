pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::learning::assign_course_role::{
    assign_course_role as run_course_role_assignment, CourseRoleAssignmentCommand,
    CourseRoleAssignmentError, CourseRoleAssignmentOutput,
};
pub(crate) use rust_learn::domain::access_control::permissions::Permissions;
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::hierarchy_queries::compare_organization_users;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::permission_queries::has_course_permission;
pub(crate) use rust_learn::infra::postgres::access_control::permission_queries::has_organization_permission;
pub(crate) use rust_learn::infra::postgres::access_control::permission_queries::has_platform_permission;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_assignments::assign_organization_role_with_hierarchy;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::establish_connection;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::learning::course_role_assignment_store::PostgresCourseRoleAssignmentStore;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::schema::{courses, organizations, users};
pub(crate) use std::cmp::Ordering;

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

pub(crate) async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_user_helper(
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

pub(crate) async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
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

pub(crate) async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
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

pub(crate) async fn get_org_admin_role_id(conn: &mut AsyncPgConnection) -> i32 {
    role_catalog_store::organization_role_id_by_name(conn, "ADMIN")
        .await
        .expect("organization admin role not found")
}

pub(crate) async fn get_org_member_role_id(conn: &mut AsyncPgConnection) -> i32 {
    role_catalog_store::organization_role_id_by_name(conn, "STUDENT")
        .await
        .expect("organization student role not found")
}

pub(crate) async fn get_course_admin_role_id(conn: &mut AsyncPgConnection) -> i32 {
    role_catalog_store::course_role_id_by_name(conn, "TEACHER")
        .await
        .expect("course teacher role not found")
}

pub(crate) async fn get_course_student_role_id(conn: &mut AsyncPgConnection) -> i32 {
    role_catalog_store::course_role_id_by_name(conn, "STUDENT")
        .await
        .expect("course student role not found")
}

pub(crate) async fn assign_org_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    org_id: i32,
    role_id: i32,
) {
    organization_role_records::assign_organization_role_to_user(conn, user_id, org_id, role_id)
        .await
        .expect("failed to assign org role");
}

pub(crate) async fn assign_course_role_with_use_case(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    target_user_id: i32,
    course_id: i32,
    role_name: &str,
) -> Result<CourseRoleAssignmentOutput, CourseRoleAssignmentError> {
    let mut store = PostgresCourseRoleAssignmentStore::new(conn);
    run_course_role_assignment(
        &mut store,
        CourseRoleAssignmentCommand {
            actor_user_id,
            course_id,
            target_user_id,
            role_name: role_name.to_string(),
        },
    )
    .await
}
