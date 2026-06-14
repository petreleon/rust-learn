use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::learning::assign_course_role::{
    assign_course_role as run_course_role_assignment, CourseRoleAssignmentCommand,
    CourseRoleAssignmentError, CourseRoleAssignmentOutput,
};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, organizations, users};
use rust_learn::infra::postgres::learning::course_role_assignment_store::PostgresCourseRoleAssignmentStore;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::infra::postgres::access_control::platform_role_records;
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

async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str, verified: bool) -> User {
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

async fn assign_org_role(conn: &mut AsyncPgConnection, user_id: i32, org_id: i32, role_id: i32) {
    UserRoleOrganization::assign(conn, user_id, org_id, role_id)
        .await
        .expect("failed to assign org role");
}

async fn assign_course_role_with_use_case(
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
