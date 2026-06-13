use chrono::NaiveDate;
use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::learning::update_course_lifecycle::{
    CourseLifecycleCommand, CourseLifecycleError, CourseLifecycleUseCase,
};
use rust_learn::db::schema::courses;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::infra::postgres::learning::course_lifecycle_use_case::PostgresCourseLifecycleUseCase;
use rust_learn::models::course::{
    Course, NewCourse, COURSE_STATUS_DRAFT, COURSE_STATUS_PUBLISHED, COURSE_STATUS_SUBMITTED,
};
use rust_learn::models::role::CourseRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::repositories::user_repository::create_user;

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

fn lifecycle_use_case(pool: &DbPool) -> PostgresCourseLifecycleUseCase {
    PostgresCourseLifecycleUseCase::new(pool.clone())
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
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

#[actix_web::test]
async fn teacher_can_move_course_through_lifecycle_states() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let teacher = create_user_helper(&mut conn, "course_lifecycle_teacher").await;
    let course = create_course(&mut conn, &unique_string("LifecycleCourse")).await;
    assert_eq!(course.lifecycle_status, COURSE_STATUS_DRAFT);
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let use_case = lifecycle_use_case(&pool);
    let submitted = use_case
        .update_course_lifecycle(CourseLifecycleCommand {
            actor_user_id: teacher.id(),
            course_id: course.id,
            status: COURSE_STATUS_SUBMITTED.to_string(),
        })
        .await
        .expect("teacher should submit course");
    assert_eq!(submitted.lifecycle_status, COURSE_STATUS_SUBMITTED);

    let published = use_case
        .update_course_lifecycle(CourseLifecycleCommand {
            actor_user_id: teacher.id(),
            course_id: course.id,
            status: COURSE_STATUS_PUBLISHED.to_string(),
        })
        .await
        .expect("teacher should publish course with PUBLISH_CONTENT");
    assert_eq!(published.lifecycle_status, COURSE_STATUS_PUBLISHED);
}

#[actix_web::test]
async fn student_cannot_publish_course() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let student = create_user_helper(&mut conn, "course_lifecycle_student").await;
    let course = create_course(&mut conn, &unique_string("LifecycleDeniedCourse")).await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    drop(conn);

    let denied = lifecycle_use_case(&pool)
        .update_course_lifecycle(CourseLifecycleCommand {
            actor_user_id: student.id(),
            course_id: course.id,
            status: COURSE_STATUS_PUBLISHED.to_string(),
        })
        .await
        .expect_err("student should not publish course");
    assert!(matches!(denied, CourseLifecycleError::PermissionDenied(_)));
}

#[actix_web::test]
async fn invalid_course_lifecycle_status_is_rejected() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let teacher = create_user_helper(&mut conn, "course_lifecycle_invalid").await;
    let course = create_course(&mut conn, &unique_string("LifecycleInvalidCourse")).await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let rejected = lifecycle_use_case(&pool)
        .update_course_lifecycle(CourseLifecycleCommand {
            actor_user_id: teacher.id(),
            course_id: course.id,
            status: "launched".to_string(),
        })
        .await
        .expect_err("invalid lifecycle status should be rejected");
    assert!(matches!(rejected, CourseLifecycleError::InvalidStatus(_)));
}
