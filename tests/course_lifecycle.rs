use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::courses;
use rust_learn::models::course::{
    Course, NewCourse, COURSE_STATUS_DRAFT, COURSE_STATUS_PUBLISHED, COURSE_STATUS_SUBMITTED,
};
use rust_learn::models::role::CourseRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::course_service::{
    update_course_lifecycle, CourseLifecycleError, CourseLifecycleUpdateRequest,
};

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
    let mut conn = setup_conn().await;
    let teacher = create_user_helper(&mut conn, "course_lifecycle_teacher").await;
    let course = create_course(&mut conn, &unique_string("LifecycleCourse")).await;
    assert_eq!(course.lifecycle_status, COURSE_STATUS_DRAFT);
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;

    let submitted = update_course_lifecycle(
        &mut conn,
        teacher.id(),
        course.id,
        CourseLifecycleUpdateRequest {
            status: COURSE_STATUS_SUBMITTED.to_string(),
        },
    )
    .await
    .expect("teacher should submit course");
    assert_eq!(submitted.lifecycle_status, COURSE_STATUS_SUBMITTED);

    let published = update_course_lifecycle(
        &mut conn,
        teacher.id(),
        course.id,
        CourseLifecycleUpdateRequest {
            status: COURSE_STATUS_PUBLISHED.to_string(),
        },
    )
    .await
    .expect("teacher should publish course with PUBLISH_CONTENT");
    assert_eq!(published.lifecycle_status, COURSE_STATUS_PUBLISHED);
}

#[actix_web::test]
async fn student_cannot_publish_course() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "course_lifecycle_student").await;
    let course = create_course(&mut conn, &unique_string("LifecycleDeniedCourse")).await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;

    let denied = update_course_lifecycle(
        &mut conn,
        student.id(),
        course.id,
        CourseLifecycleUpdateRequest {
            status: COURSE_STATUS_PUBLISHED.to_string(),
        },
    )
    .await
    .expect_err("student should not publish course");
    assert!(matches!(denied, CourseLifecycleError::PermissionDenied(_)));
}

#[actix_web::test]
async fn invalid_course_lifecycle_status_is_rejected() {
    let mut conn = setup_conn().await;
    let teacher = create_user_helper(&mut conn, "course_lifecycle_invalid").await;
    let course = create_course(&mut conn, &unique_string("LifecycleInvalidCourse")).await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;

    let rejected = update_course_lifecycle(
        &mut conn,
        teacher.id(),
        course.id,
        CourseLifecycleUpdateRequest {
            status: "launched".to_string(),
        },
    )
    .await
    .expect_err("invalid lifecycle status should be rejected");
    assert!(matches!(rejected, CourseLifecycleError::InvalidStatus(_)));
}
