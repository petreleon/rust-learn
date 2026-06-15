pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::db::{establish_connection, DbPool};
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;

pub(crate) use actix_service::Service;
pub(crate) use chrono::NaiveDate;
pub(crate) use rust_learn::application::access_control::check_permission::AccessDecisionService;
pub(crate) use rust_learn::application::access_control::compare_hierarchy::HierarchyCheckService;
pub(crate) use rust_learn::application::access_control::list_roles::RoleCatalogUseCase;
pub(crate) use rust_learn::application::identity::assign_platform_role::PlatformRoleAssignmentUseCase;
pub(crate) use rust_learn::application::identity::get_user_profile::UserProfileReadUseCase;
pub(crate) use rust_learn::application::identity::list_users::UserListUseCase;
pub(crate) use rust_learn::application::learning::discover_courses::CourseDiscoveryUseCase;
pub(crate) use rust_learn::application::learning::get_course::CourseReadUseCase;
pub(crate) use rust_learn::application::learning::list_course_organizations::CourseOrganizationsUseCase;
pub(crate) use rust_learn::db::schema::{courses, organizations};
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_use_case::PostgresRoleCatalogUseCase;
pub(crate) use rust_learn::infra::postgres::identity::platform_role_assignment_use_case::PostgresPlatformRoleAssignmentUseCase;
pub(crate) use rust_learn::infra::postgres::identity::user_list_use_case::PostgresUserListUseCase;
pub(crate) use rust_learn::infra::postgres::identity::user_profile_read_use_case::PostgresUserProfileReadUseCase;
pub(crate) use rust_learn::infra::postgres::learning::course_discovery_use_case::PostgresCourseDiscoveryUseCase;
pub(crate) use rust_learn::infra::postgres::learning::course_organization_use_case::PostgresCourseOrganizationsUseCase;
pub(crate) use rust_learn::infra::postgres::learning::course_read_use_case::PostgresCourseReadUseCase;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
use std::sync::Arc;

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
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

pub(crate) fn generate_token(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to generate token")
}

pub(crate) fn permission_check_use_case_data(pool: &DbPool) -> web::Data<AccessDecisionService> {
    web::Data::new(Arc::new(pool.clone()))
}

pub(crate) fn hierarchy_check_use_case_data(pool: &DbPool) -> web::Data<HierarchyCheckService> {
    web::Data::new(Arc::new(pool.clone()))
}

pub(crate) fn role_catalog_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn RoleCatalogUseCase>> {
    web::Data::new(Arc::new(PostgresRoleCatalogUseCase::new(pool.clone())))
}

pub(crate) fn platform_role_assignment_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn PlatformRoleAssignmentUseCase>> {
    web::Data::new(Arc::new(PostgresPlatformRoleAssignmentUseCase::new(
        pool.clone(),
        rust_learn::infra::notifications::NotificationsState::new(pool.clone()),
    )))
}

pub(crate) fn user_list_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn UserListUseCase>> {
    web::Data::new(Arc::new(PostgresUserListUseCase::new(pool.clone())))
}

pub(crate) fn user_profile_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn UserProfileReadUseCase>> {
    web::Data::new(Arc::new(PostgresUserProfileReadUseCase::new(pool.clone())))
}

pub(crate) fn course_discovery_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn CourseDiscoveryUseCase>> {
    web::Data::new(Arc::new(PostgresCourseDiscoveryUseCase::new(pool.clone())))
}

pub(crate) fn course_organizations_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn CourseOrganizationsUseCase>> {
    web::Data::new(Arc::new(PostgresCourseOrganizationsUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn course_read_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn CourseReadUseCase>> {
    web::Data::new(Arc::new(PostgresCourseReadUseCase::new(pool.clone())))
}

pub(crate) fn response_status<B>(
    result: Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
) -> StatusCode {
    match result {
        Ok(resp) => resp.status(),
        Err(e) => e.error_response().status(),
    }
}

pub(crate) async fn force_assign_platform_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("role not found");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("assign failed");
}

pub(crate) async fn force_assign_org_role(
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
        .expect("assign failed");
}

pub(crate) async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("role not found");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("assign failed");
}
