use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rust_learn::db::establish_connection;
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::utils::db_utils::course::{assign_role_to_user_in_course, user_permission_course_request};
use rust_learn::config::constants::roles::Roles;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::models::course::{NewCourse, Course};
use rust_learn::db::schema::courses;
use chrono::NaiveDate;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos();
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

    assign_role_to_user_in_course(&mut conn, user.id(), course.id, Roles::TEACHER)
        .expect("failed to assign TEACHER role");

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

    assign_role_to_user_in_course(&mut conn, user.id(), course.id, Roles::STUDENT)
        .expect("failed to assign STUDENT role");

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
