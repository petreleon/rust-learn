use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::learning::create_course::{
    CourseCreationCommand, CourseCreationError, CourseCreationUseCase,
};
use rust_learn::infra::postgres::access_control::organization_role_records;
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
use rust_learn::infra::postgres::learning::course_creation_use_case::PostgresCourseCreationUseCase;
use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
use rust_learn::infra::postgres::models::user::User;
use rust_learn::infra::postgres::schema::{
    courses_organizations, organizations, pending_course_organization_invites,
};
use rust_learn::infra::postgres::{establish_connection, DbPool};

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

fn setup_pool() -> DbPool {
    let _ = dotenvy::dotenv();
    establish_connection()
}

async fn setup_conn(pool: &DbPool) -> Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

fn course_creation_use_case(pool: &DbPool) -> PostgresCourseCreationUseCase {
    PostgresCourseCreationUseCase::new(pool.clone())
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
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name)
        .await
        .expect("organization role not found");
    organization_role_records::assign_organization_role_to_user(
        conn,
        user_id,
        organization_id,
        role_id,
    )
    .await
    .expect("failed to assign organization role");
}

#[actix_web::test]
async fn organization_admin_can_create_course_for_owned_organization() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let organization = create_organization(&mut conn, &unique_string("OrgCourseCreation")).await;
    let invited_organization =
        create_organization(&mut conn, &unique_string("InvitedOrgCourseCreation")).await;
    let org_admin = create_user_helper(&mut conn, "org_course_creator").await;
    force_assign_organization_role(&mut conn, org_admin.id(), organization.id, "ADMIN").await;
    drop(conn);

    let course = course_creation_use_case(&pool)
        .create_course(CourseCreationCommand {
            actor_user_id: org_admin.id(),
            title: unique_string("OrgOwnedCourse"),
            organization_ids: vec![organization.id, invited_organization.id],
        })
        .await
        .expect("organization admin should create course for owned organization");

    let mut conn = setup_conn(&pool).await;
    let linked_org_id = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course.id))
        .select(courses_organizations::organization_id)
        .first::<i32>(&mut conn)
        .await
        .expect("course should be linked to organization");
    assert_eq!(linked_org_id, organization.id);

    let pending_invite = pending_course_organization_invites::table
        .filter(pending_course_organization_invites::course_id.eq(course.id))
        .select((
            pending_course_organization_invites::organization_id,
            pending_course_organization_invites::order,
        ))
        .first::<(i32, i32)>(&mut conn)
        .await
        .expect("second organization should be pending invite");
    assert_eq!(pending_invite, (invited_organization.id, 1));
}

#[actix_web::test]
async fn user_without_platform_or_organization_permission_cannot_create_course() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let organization =
        create_organization(&mut conn, &unique_string("DeniedOrgCourseCreation")).await;
    let user = create_user_helper(&mut conn, "org_course_denied").await;
    drop(conn);

    let denied = course_creation_use_case(&pool)
        .create_course(CourseCreationCommand {
            actor_user_id: user.id(),
            title: unique_string("DeniedCourse"),
            organization_ids: vec![organization.id],
        })
        .await
        .expect_err("user without scoped CREATE_COURSE should be denied");
    assert!(matches!(denied, CourseCreationError::PermissionDenied(_)));
}
