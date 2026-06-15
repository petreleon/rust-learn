pub(crate) use chrono::NaiveDate;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::learning::assign_course_role::{
    assign_course_role as run_course_role_assignment, CourseRoleAssignmentCommand,
    CourseRoleAssignmentError, CourseRoleAssignmentOutput,
};
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::establish_connection;
pub(crate) use rust_learn::db::schema::courses;
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::permission_queries::has_course_permission;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::learning::course_role_assignment_store::PostgresCourseRoleAssignmentStore;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};

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

pub(crate) async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
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
        .expect("Error creating course")
}

pub(crate) async fn force_assign_role(
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
        .expect("force assign failed");
}

pub(crate) async fn assign_course_role_via_use_case(
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

pub(crate) async fn create_user_helper(
    conn: &mut AsyncPgConnection,
    name_suffix: &str,
) -> rust_learn::infra::postgres::models::user::User {
    let suffix = unique_string(name_suffix);
    let email = format!("user_{}@example.com", suffix);
    rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user(
        conn,
        &format!("User {}", suffix),
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password",
    )
    .await
    .expect("failed to create user")
}
