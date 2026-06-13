use actix_web::{http::StatusCode, test, web, App};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::user::User;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;

use actix_service::Service;
use chrono::NaiveDate;
use rust_learn::application::access_control::list_roles::RoleCatalogUseCase;
use rust_learn::application::learning::discover_courses::CourseDiscoveryUseCase;
use rust_learn::application::learning::get_course::CourseReadUseCase;
use rust_learn::application::learning::list_course_organizations::CourseOrganizationsUseCase;
use rust_learn::db::schema::{courses, organizations};
use rust_learn::infra::postgres::access_control::role_catalog_use_case::PostgresRoleCatalogUseCase;
use rust_learn::infra::postgres::learning::course_discovery_use_case::PostgresCourseDiscoveryUseCase;
use rust_learn::infra::postgres::learning::course_read_use_case::PostgresCourseReadUseCase;
use rust_learn::infra::postgres::learning::course_organization_use_case::PostgresCourseOrganizationsUseCase;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform; // Import Service trait for .call()
use std::sync::Arc;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
    let email = unique_string(name) + "@example.com";
    create_user(
        conn,
        name,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password",
    )
    .await
    .expect("failed to create user")
}

fn generate_token(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to generate token")
}

fn role_catalog_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn RoleCatalogUseCase>> {
    web::Data::new(Arc::new(PostgresRoleCatalogUseCase::new(pool.clone())))
}

fn course_discovery_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn CourseDiscoveryUseCase>> {
    web::Data::new(Arc::new(PostgresCourseDiscoveryUseCase::new(pool.clone())))
}

fn course_organizations_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn CourseOrganizationsUseCase>> {
    web::Data::new(Arc::new(PostgresCourseOrganizationsUseCase::new(pool.clone())))
}

fn course_read_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn CourseReadUseCase>> {
    web::Data::new(Arc::new(PostgresCourseReadUseCase::new(pool.clone())))
}

fn response_status<B>(
    result: Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
) -> StatusCode {
    match result {
        Ok(resp) => resp.status(),
        Err(e) => e.error_response().status(),
    }
}

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("role not found");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("assign failed");
}

async fn force_assign_org_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    org_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("role not found");
    UserRoleOrganization::assign(conn, user_id, org_id, role_id)
        .await
        .expect("assign failed");
}

async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("assign failed");
}
