use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::learning::assign_course_role::{
    assign_course_role as run_course_role_assignment, CourseRoleAssignmentCommand,
    CourseRoleAssignmentError, CourseRoleAssignmentOutput,
};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::courses;
use rust_learn::infra::postgres::learning::course_role_assignment_store::PostgresCourseRoleAssignmentStore;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::infra::postgres::access_control::course_role_records;
use rust_learn::repositories::course_repository::user_permission_course_request;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;

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
        .expect("Error creating course")
}

async fn force_assign_role(
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

async fn assign_course_role_via_use_case(
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

async fn create_user_helper(
    conn: &mut AsyncPgConnection,
    name_suffix: &str,
) -> rust_learn::models::user::User {
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

#[actix_web::test]
async fn teacher_has_permissions() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("TeacherCourse");
    let course = create_course(&mut conn, &course_title).await;

    let email = unique_string("teacher") + "@example.com";
    let user = create_user(
        &mut conn,
        "Teacher Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1980, 5, 5).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");

    force_assign_role(&mut conn, user.id(), course.id, "TEACHER").await;

    let allowed_permissions = [Permissions::MANAGE_COURSE_SETTINGS];

    for p in allowed_permissions {
        let has_perm =
            user_permission_course_request(&mut conn, user.id(), course.id, &p.to_string())
                .await
                .expect("permission query failed");
        assert!(has_perm, "TEACHER should have permission: {:?}", p);
    }
}
