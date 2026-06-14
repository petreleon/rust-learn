use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::teacher_applications::get_my_application::{
    TeacherApplicationSelfOutput, TeacherApplicationSelfUseCase,
};
use rust_learn::application::teacher_applications::list_application_audit::TeacherApplicationAuditUseCase;
use rust_learn::application::teacher_applications::list_applications::{
    TeacherApplicationListQuery, TeacherApplicationListUseCase,
};
use rust_learn::application::teacher_applications::TeacherApplicationOutput;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, organizations, platform_roles, role_permission_platform};
use rust_learn::domain::teacher_applications::status::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::OrganizationRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::repositories::course_repository::user_permission_course_request;
use rust_learn::repositories::organization_repository::user_permission_organization_request;
use rust_learn::repositories::platform_repository::assign_role_to_user;
use rust_learn::repositories::platform_repository::user_permission_platform_request;
use rust_learn::infra::postgres::teacher_applications::teacher_application_audit_use_case::PostgresTeacherApplicationAuditUseCase;
use rust_learn::infra::postgres::teacher_applications::teacher_application_list_use_case::PostgresTeacherApplicationListUseCase;
use rust_learn::infra::postgres::teacher_applications::teacher_application_self_use_case::PostgresTeacherApplicationSelfUseCase;
use rust_learn::repositories::teacher_application_repository::list_audit_events;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
enum TeacherApplicationError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidTransition(String),
    NotFound,
    Database(String),
}

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

async fn get_my_application(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<TeacherApplicationSelfOutput, rust_learn::application::teacher_applications::get_my_application::TeacherApplicationSelfError>
{
    let pool = establish_connection();
    PostgresTeacherApplicationSelfUseCase::new(pool)
        .get_my_application(actor_user_id)
        .await
}

#[derive(Debug, Clone, Default)]
struct ListTeacherApplicationsRequest {
    status: Option<String>,
    applicant_user_id: Option<i32>,
    organization_sponsor_id: Option<i32>,
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list_applications(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: ListTeacherApplicationsRequest,
) -> Result<Vec<TeacherApplicationOutput>, rust_learn::application::teacher_applications::list_applications::TeacherApplicationListError>
{
    PostgresTeacherApplicationListUseCase::new(establish_connection())
        .list_applications(TeacherApplicationListQuery {
            actor_user_id,
            applicant_user_id: request.applicant_user_id,
            limit: request.limit,
            offset: request.offset,
            organization_sponsor_id: request.organization_sponsor_id,
            status: request.status,
        })
        .await
}

fn teacher_application_self_data() -> web::Data<Arc<dyn TeacherApplicationSelfUseCase>> {
    web::Data::new(
        Arc::new(PostgresTeacherApplicationSelfUseCase::new(establish_connection()))
            as Arc<dyn TeacherApplicationSelfUseCase>,
    )
}

fn teacher_application_audit_data() -> web::Data<Arc<dyn TeacherApplicationAuditUseCase>> {
    web::Data::new(
        Arc::new(PostgresTeacherApplicationAuditUseCase::new(establish_connection()))
            as Arc<dyn TeacherApplicationAuditUseCase>,
    )
}

fn teacher_application_list_data() -> web::Data<Arc<dyn TeacherApplicationListUseCase>> {
    web::Data::new(
        Arc::new(PostgresTeacherApplicationListUseCase::new(establish_connection()))
            as Arc<dyn TeacherApplicationListUseCase>,
    )
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
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
    let new_org = NewOrganization {
        name: name.to_string(),
        website_link: None,
        profile_url: None,
    };

    diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    let new_course = NewCourse {
        title: title.to_string(),
        description: None,
        topics: None,
        prerequisites: None,
    };

    diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result(conn)
        .await
        .expect("failed to create course")
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
        .expect("failed to force assign organization role");
}
