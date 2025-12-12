use diesel::prelude::*;
use rust_learn::db::establish_connection;
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::utils::db_utils::course::{assign_role_to_user_in_course, user_permission_course_request};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::models::course::{NewCourse, Course};
use rust_learn::db::schema::courses;
use chrono::NaiveDate;
use rust_learn::models::role::CourseRole;
use rust_learn::models::user_role_course::UserRoleCourse;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn() -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get().await.expect("failed to get DB connection from pool")
}

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    let new_course = NewCourse {
        title: title.to_string(),
    };

    diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result(conn)
        .await
        .expect("Error creating course")
}

async fn force_assign_role(conn: &mut AsyncPgConnection, user_id: i32, course_id: i32, role_name: &str) {
    let role_id = CourseRole::find_by_name(role_name, conn).await.expect("role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id).await.expect("force assign failed");
}

async fn create_user_helper(conn: &mut AsyncPgConnection, name_suffix: &str) -> rust_learn::models::user::User {
    let suffix = unique_string(name_suffix);
    let email = format!("user_{}@example.com", suffix);
    rust_learn::utils::db_utils::authentication_registration::create_user(
        conn,
        &format!("User {}", suffix),
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password"
    ).await.expect("failed to create user")
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

    let allowed_permissions = [
        Permissions::MANAGE_COURSE_SETTINGS,
    ];

    for p in allowed_permissions {
        let has_perm = user_permission_course_request(&mut conn, user.id(), course.id, &p.to_string())
            .await
            .expect("permission query failed");
        assert!(has_perm, "TEACHER should have permission: {:?}", p);
    }
}

#[actix_web::test]
async fn student_has_limited_permissions() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("StudentCourse");
    let course = create_course(&mut conn, &course_title).await;

    let email = unique_string("student") + "@example.com";
    let user = create_user(
        &mut conn,
        "Student Test",
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");

    force_assign_role(&mut conn, user.id(), course.id, "STUDENT").await;

    // Assuming STUDENT does not have MANAGE_COURSE_SETTINGS
    let denied_permissions = [
        Permissions::MANAGE_COURSE_SETTINGS,
    ];

    for p in denied_permissions {
        let has_perm = user_permission_course_request(&mut conn, user.id(), course.id, &p.to_string())
            .await
            .expect("permission query failed");
        assert!(!has_perm, "STUDENT should NOT have permission: {:?}", p);
    }
}

#[actix_web::test]
async fn assign_hierarchy_check_success() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("HierCourseSuccess");
    let course = create_course(&mut conn, &course_title).await;

    // 1. Create a TEACHER (Assigner)
    let teacher_user = create_user_helper(&mut conn, "teacher_assigner").await;
    force_assign_role(&mut conn, teacher_user.id(), course.id, "TEACHER").await;

    // 2. Create a fresh user (Assignee)
    let student_user = create_user_helper(&mut conn, "new_student").await;

    // 3. Teacher assigns STUDENT role
    let result = assign_role_to_user_in_course(
        &mut conn, 
        teacher_user.id(), 
        student_user.id(), 
        course.id, 
        "STUDENT"
    ).await;
    assert!(result.is_ok(), "TEACHER should be able to assign STUDENT");
}

#[actix_web::test]
async fn assign_hierarchy_check_fail_assigning_higher_role() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("HierCourseFailRole");
    let course = create_course(&mut conn, &course_title).await;

    // 1. Create a STUDENT (Assigner)
    let student_assigner = create_user_helper(&mut conn, "student_assigner").await;
    force_assign_role(&mut conn, student_assigner.id(), course.id, "STUDENT").await;

    // 2. Create a fresh user
    let new_user = create_user_helper(&mut conn, "target_user").await;

    // 3. Student tries to assign TEACHER
    let result = assign_role_to_user_in_course(
        &mut conn, 
        student_assigner.id(), 
        new_user.id(), 
        course.id, 
        "TEACHER"
    ).await;
    assert!(result.is_err(), "STUDENT should NOT be able to assign TEACHER");
}
