use chrono::NaiveDate;
use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::learning::update_course::{
    CourseUpdateCommand, CourseUpdateError, CourseUpdateUseCase,
};
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::schema::courses;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::infra::postgres::access_control::course_role_records;
use rust_learn::infra::postgres::access_control::role_assignments::assign_platform_role_to_user;
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
use rust_learn::infra::postgres::learning::course_update_use_case::PostgresCourseUpdateUseCase;
use rust_learn::infra::postgres::models::course::{Course, NewCourse};
use rust_learn::infra::postgres::models::user::User;

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

fn course_update_use_case(pool: &DbPool) -> PostgresCourseUpdateUseCase {
    PostgresCourseUpdateUseCase::new(pool.clone())
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

async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("course role not found");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

#[actix_web::test]
async fn course_teacher_can_edit_course_settings() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let teacher = create_user_helper(&mut conn, "course_edit_teacher").await;
    let course = create_course(&mut conn, &unique_string("EditableCourse")).await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let updated = course_update_use_case(&pool)
        .update_course(CourseUpdateCommand {
            actor_user_id: teacher.id(),
            course_id: course.id,
            title: Some("Teacher Updated Course".to_string()),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .await
        .expect("course teacher should update course");
    assert_eq!(updated.title, "Teacher Updated Course");
}

#[actix_web::test]
async fn platform_modify_course_permission_can_edit_without_course_role() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_user_helper(&mut conn, "course_edit_platform").await;
    let course = create_course(&mut conn, &unique_string("PlatformEditableCourse")).await;
    assign_platform_role_to_user(&mut conn, platform_admin.id(), Roles::SUPER_ADMIN)
        .await
        .expect("failed to assign SUPER_ADMIN role");
    drop(conn);

    let updated = course_update_use_case(&pool)
        .update_course(CourseUpdateCommand {
            actor_user_id: platform_admin.id(),
            course_id: course.id,
            title: Some("Platform Updated Course".to_string()),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .await
        .expect("platform MODIFY_COURSE should update course");
    assert_eq!(updated.title, "Platform Updated Course");
}

#[actix_web::test]
async fn course_student_cannot_edit_course_settings() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let student = create_user_helper(&mut conn, "course_edit_student").await;
    let course = create_course(&mut conn, &unique_string("DeniedEditableCourse")).await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    drop(conn);

    let denied = course_update_use_case(&pool)
        .update_course(CourseUpdateCommand {
            actor_user_id: student.id(),
            course_id: course.id,
            title: Some("Student Update".to_string()),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .await
        .expect_err("course student should not update course");
    assert!(matches!(denied, CourseUpdateError::PermissionDenied(_)));
}
