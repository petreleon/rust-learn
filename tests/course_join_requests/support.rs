pub(crate) use chrono::NaiveDate;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::learning::course_enrollment::{
    decide_course_join_request as decide_course_join_request_with_store,
    remove_course_enrollment as remove_course_enrollment_with_store,
    request_course_join as request_course_join_with_store, CourseEnrollmentError,
    CourseEnrollmentRemovalOutput, CourseJoinRequestOutput, DecideCourseJoinCommand,
    RemoveCourseEnrollmentCommand, RequestCourseJoinCommand,
};
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::establish_connection;
pub(crate) use rust_learn::db::schema::{courses, courses_organizations, organizations};
pub(crate) use rust_learn::domain::learning::enrollment::status::{
    COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED,
};
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::permission_queries::has_course_permission;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::learning::course_enrollment_store::PostgresCourseEnrollmentStore;
pub(crate) use rust_learn::models::course::{Course, NewCourse};
pub(crate) use rust_learn::models::courses_organizations::NewCourseOrganization;
pub(crate) use rust_learn::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::models::user::User;

#[derive(Debug, Clone)]
pub(crate) struct CourseJoinDecisionRequest {
    pub(crate) status: String,
    pub(crate) decision_reason: Option<String>,
}

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

pub(crate) async fn request_course_join(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<CourseJoinRequestOutput, CourseEnrollmentError> {
    let mut store = PostgresCourseEnrollmentStore::new(conn);
    request_course_join_with_store(
        &mut store,
        RequestCourseJoinCommand {
            actor_user_id,
            course_id,
        },
    )
    .await
}

pub(crate) async fn decide_course_join_request(
    conn: &mut AsyncPgConnection,
    reviewer_user_id: i32,
    course_id: i32,
    request_id: i64,
    request: CourseJoinDecisionRequest,
) -> Result<CourseJoinRequestOutput, CourseEnrollmentError> {
    let mut store = PostgresCourseEnrollmentStore::new(conn);
    decide_course_join_request_with_store(
        &mut store,
        DecideCourseJoinCommand {
            reviewer_user_id,
            course_id,
            request_id,
            status: request.status,
            decision_reason: request.decision_reason,
        },
    )
    .await
    .map(|output| output.join_request)
}

pub(crate) async fn remove_course_enrollment(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    target_user_id: i32,
) -> Result<CourseEnrollmentRemovalOutput, CourseEnrollmentError> {
    let mut store = PostgresCourseEnrollmentStore::new(conn);
    remove_course_enrollment_with_store(
        &mut store,
        RemoveCourseEnrollmentCommand {
            actor_user_id,
            course_id,
            target_user_id,
        },
    )
    .await
}

pub(crate) async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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

pub(crate) async fn link_course_to_organization(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course to organization");
}
