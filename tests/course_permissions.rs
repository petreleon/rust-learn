use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rust_learn::db::establish_connection;
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::utils::db_utils::course::{assign_role_to_user_in_course, user_permission_course_request};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::models::course::{NewCourse, Course};
use rust_learn::db::schema::courses;
use chrono::NaiveDate;
use rust_learn::models::role::CourseRole;
use rust_learn::models::user_role_course::UserRoleCourse;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

fn setup_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get().expect("failed to get DB connection from pool")
}

fn create_course(conn: &mut PgConnection, title: &str) -> Course {
    let new_course = NewCourse {
        title: title.to_string(),
    };

    diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result(conn)
        .expect("Error creating course")
}

fn force_assign_role(conn: &mut PgConnection, user_id: i32, course_id: i32, role_name: &str) {
    let role_id = CourseRole::find_by_name(role_name, conn).expect("role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id).expect("force assign failed");
}

fn create_user_helper(conn: &mut PgConnection, name_suffix: &str) -> rust_learn::models::user::User {
    let suffix = unique_string(name_suffix);
    let email = format!("user_{}@example.com", suffix);
    rust_learn::utils::db_utils::authentication_registration::create_user(
        conn,
        &format!("User {}", suffix),
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password"
    ).expect("failed to create user")
}

#[test]
fn teacher_has_permissions() {
    let mut conn = setup_conn();
    let course_title = unique_string("TeacherCourse");
    let course = create_course(&mut conn, &course_title);

    let email = unique_string("teacher") + "@example.com";
    let user = create_user(
        &mut conn,
        "Teacher Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1980, 5, 5).unwrap()),
        "password123",
    )
    .expect("failed to create user");

    force_assign_role(&mut conn, user.id(), course.id, "TEACHER");

    let allowed_permissions = [
        Permissions::MANAGE_COURSE_SETTINGS,
    ];

    for p in allowed_permissions {
        let has_perm = user_permission_course_request(&mut conn, user.id(), course.id, &p.to_string())
            .expect("permission query failed");
        assert!(has_perm, "TEACHER should have permission: {:?}", p);
    }
}

#[test]
fn student_has_limited_permissions() {
    let mut conn = setup_conn();
    let course_title = unique_string("StudentCourse");
    let course = create_course(&mut conn, &course_title);

    let email = unique_string("student") + "@example.com";
    let user = create_user(
        &mut conn,
        "Student Test",
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password123",
    )
    .expect("failed to create user");

    force_assign_role(&mut conn, user.id(), course.id, "STUDENT");

    // Assuming STUDENT does not have MANAGE_COURSE_SETTINGS
    let denied_permissions = [
        Permissions::MANAGE_COURSE_SETTINGS,
    ];

    for p in denied_permissions {
        let has_perm = user_permission_course_request(&mut conn, user.id(), course.id, &p.to_string())
            .expect("permission query failed");
        assert!(!has_perm, "STUDENT should NOT have permission: {:?}", p);
    }
}

#[test]
fn assign_hierarchy_check_success() {
    let mut conn = setup_conn();
    let course_title = unique_string("HierCourseSuccess");
    let course = create_course(&mut conn, &course_title);

    // 1. Create a TEACHER (Assigner)
    let teacher_user = create_user_helper(&mut conn, "teacher_assigner");
    force_assign_role(&mut conn, teacher_user.id(), course.id, "TEACHER");

    // 2. Create a fresh user (Assignee)
    let student_user = create_user_helper(&mut conn, "new_student");

    // 3. Teacher assigns STUDENT role
    let result = assign_role_to_user_in_course(
        &mut conn, 
        teacher_user.id(), 
        student_user.id(), 
        course.id, 
        "STUDENT"
    );
    assert!(result.is_ok(), "TEACHER should be able to assign STUDENT");
}

#[test]
fn assign_hierarchy_check_fail_assigning_higher_role() {
    let mut conn = setup_conn();
    let course_title = unique_string("HierCourseFailRole");
    let course = create_course(&mut conn, &course_title);

    // 1. Create a STUDENT (Assigner)
    let student_assigner = create_user_helper(&mut conn, "student_assigner");
    force_assign_role(&mut conn, student_assigner.id(), course.id, "STUDENT");

    // 2. Create a fresh user
    let new_user = create_user_helper(&mut conn, "target_user");

    // 3. Student tries to assign TEACHER
    let result = assign_role_to_user_in_course(
        &mut conn, 
        student_assigner.id(), 
        new_user.id(), 
        course.id, 
        "TEACHER"
    );
    assert!(result.is_err(), "STUDENT should NOT be able to assign TEACHER");
}
