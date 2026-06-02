use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::courses;
use rust_learn::models::course::{Course, NewCourse, UpdateCourse};
use rust_learn::models::role::CourseRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::repositories::platform_repository::assign_role_to_user;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::course_service::{update_course_for_actor, CourseUpdateError};

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

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
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
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

#[actix_web::test]
async fn course_teacher_can_edit_course_settings() {
    let mut conn = setup_conn().await;
    let teacher = create_user_helper(&mut conn, "course_edit_teacher").await;
    let course = create_course(&mut conn, &unique_string("EditableCourse")).await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;

    let updated = update_course_for_actor(
        &mut conn,
        teacher.id(),
        course.id,
        UpdateCourse {
            title: Some("Teacher Updated Course".to_string()),
        },
    )
    .await
    .expect("course teacher should update course");
    assert_eq!(updated.title, "Teacher Updated Course");
}

#[actix_web::test]
async fn platform_modify_course_permission_can_edit_without_course_role() {
    let mut conn = setup_conn().await;
    let platform_admin = create_user_helper(&mut conn, "course_edit_platform").await;
    let course = create_course(&mut conn, &unique_string("PlatformEditableCourse")).await;
    assign_role_to_user(&mut conn, platform_admin.id(), Roles::SUPER_ADMIN)
        .await
        .expect("failed to assign SUPER_ADMIN role");

    let updated = update_course_for_actor(
        &mut conn,
        platform_admin.id(),
        course.id,
        UpdateCourse {
            title: Some("Platform Updated Course".to_string()),
        },
    )
    .await
    .expect("platform MODIFY_COURSE should update course");
    assert_eq!(updated.title, "Platform Updated Course");
}

#[actix_web::test]
async fn course_student_cannot_edit_course_settings() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "course_edit_student").await;
    let course = create_course(&mut conn, &unique_string("DeniedEditableCourse")).await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;

    let denied = update_course_for_actor(
        &mut conn,
        student.id(),
        course.id,
        UpdateCourse {
            title: Some("Student Update".to_string()),
        },
    )
    .await
    .expect_err("course student should not update course");
    assert!(matches!(denied, CourseUpdateError::PermissionDenied(_)));
}
