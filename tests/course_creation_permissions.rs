use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses_organizations, organizations};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::OrganizationRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::course_service::{
    create_course_with_invites_for_actor, CourseCreationError,
};

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
    create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user")
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

async fn force_assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role not found");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

#[actix_web::test]
async fn organization_admin_can_create_course_for_owned_organization() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("OrgCourseCreation")).await;
    let org_admin = create_user_helper(&mut conn, "org_course_creator").await;
    force_assign_organization_role(&mut conn, org_admin.id(), organization.id, "ADMIN").await;

    let course = create_course_with_invites_for_actor(
        &mut conn,
        org_admin.id(),
        unique_string("OrgOwnedCourse"),
        vec![organization.id],
    )
    .await
    .expect("organization admin should create course for owned organization");

    let linked_org_id = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course.id))
        .select(courses_organizations::organization_id)
        .first::<i32>(&mut conn)
        .await
        .expect("course should be linked to organization");
    assert_eq!(linked_org_id, organization.id);
}

#[actix_web::test]
async fn user_without_platform_or_organization_permission_cannot_create_course() {
    let mut conn = setup_conn().await;
    let organization =
        create_organization(&mut conn, &unique_string("DeniedOrgCourseCreation")).await;
    let user = create_user_helper(&mut conn, "org_course_denied").await;

    let denied = create_course_with_invites_for_actor(
        &mut conn,
        user.id(),
        unique_string("DeniedCourse"),
        vec![organization.id],
    )
    .await
    .expect_err("user without scoped CREATE_COURSE should be denied");
    assert!(matches!(denied, CourseCreationError::PermissionDenied(_)));
}
