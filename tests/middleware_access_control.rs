use actix_web::{test, App, web};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::models::user::User;
use rust_learn::utils::jwt_utils::{generate_jwt, UserJWT};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::course::{NewCourse, Course};
use rust_learn::db::schema::{organizations, courses};
use rust_learn::models::role::{PlatformRole, OrganizationRole, CourseRole};
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_course::UserRoleCourse;
use chrono::NaiveDate;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

fn setup_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get().expect("failed to get DB connection from pool")
}

fn create_test_user(conn: &mut PgConnection, name: &str) -> User {
    let email = unique_string(name) + "@example.com";
    create_user(
        conn,
        name,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password",
    )
    .expect("failed to create user")
}

fn generate_token(user_id: i32) -> String {
    let user_jwt = UserJWT { user_id, email: "test@example.com".to_string() };
    generate_jwt(user_jwt).expect("failed to generate token")
}

fn force_assign_platform_role(conn: &mut PgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn).expect("role not found");
    UserRolePlatform::assign(conn, user_id, role_id).expect("assign failed");
}

fn force_assign_org_role(conn: &mut PgConnection, user_id: i32, org_id: i32, role_name: &str) {
    let role_id = OrganizationRole::find_by_name(role_name, conn).expect("role not found");
    UserRoleOrganization::assign(conn, user_id, org_id, role_id).expect("assign failed");
}

fn force_assign_course_role(conn: &mut PgConnection, user_id: i32, course_id: i32, role_name: &str) {
    let role_id = CourseRole::find_by_name(role_name, conn).expect("role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id).expect("assign failed");
}

#[actix_web::test]
async fn test_platform_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let setup_pool = pool.clone();

    // Setup Users
    let mut conn = setup_conn();
    let admin = create_test_user(&mut conn, "admin");
    let target_user = create_test_user(&mut conn, "target");
    let unprivileged = create_test_user(&mut conn, "unpriv");

    // Assign ADMIN role to admin user (Assuming ADMIN has ASSIGN_ROLES_TO_USER)
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN");

    let admin_token = generate_token(admin.id());
    let unprivileged_token = generate_token(unprivileged.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::users::user_scope())
    ).await;

    // 1. Unprivileged user tries to assign role -> Should Fail (403 or 401 if logic matches)
    // Middleware returns 403 Forbidden ("User does not have the required permission")
    let req = test::TestRequest::post()
        .uri(&format!("/user/{}/role", target_user.id()))
        .insert_header(("Authorization", format!("Bearer {}", unprivileged_token)))
        .set_json(serde_json::json!({ "role_name": "STUDENT" })) // Role doesn't matter for permission check
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN, "Unprivileged user should be forbidden");

    // 2. Admin user tries to assign role -> Should Succeed (or at least pass middleware)
    // Note: The handler also checks hierarchy, but middleware check comes first.
    // If middleware passes, it reaches handler. Handler might fail on business logic, but not "Missing permission".
    let req = test::TestRequest::post()
        .uri(&format!("/user/{}/role", target_user.id()))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(serde_json::json!({ "role_name": "USER" })) 
        .to_request();
    let resp = test::call_service(&app, req).await;
    // It might be 200 OK or 403 Forbidden (Hierarchy) or 400 Bad Request.
    // But importantly, it is NOT "User does not have the required permission" from middleware.
    // If it was middleware block, body would be specific.
    // Let's check status != 403 OR check body if 403.
    // Actually, Admin assigning USER role to a fresh user should be OK strictly speaking if Admin > User.
    // Let's assume OK.
    assert!(resp.status().is_success() || resp.status() == actix_web::http::StatusCode::FORBIDDEN, 
        "Admin should pass middleware (status: {})", resp.status());
    
    // If Forbidden, verify it's NOT the middleware message
    if resp.status() == actix_web::http::StatusCode::FORBIDDEN {
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(!body_str.contains("User does not have the required permission"), "Should pass middleware permission check");
    }
}

#[actix_web::test]
async fn test_organization_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    
    // Setup Data
    let mut conn = setup_conn();
    let owner = create_test_user(&mut conn, "org_owner");
    let stranger = create_test_user(&mut conn, "stranger");
    
    let new_org = NewOrganization { name: unique_string("TestOrg"), website_link: None, profile_url: None };
    let org = diesel::insert_into(organizations::table).values(&new_org).get_result::<Organization>(&mut conn).unwrap();

    force_assign_org_role(&mut conn, owner.id(), org.id, "OWNER"); 

    let owner_token = generate_token(owner.id());
    let stranger_token = generate_token(stranger.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope())
    ).await;

    // 1. Stranger (no role) tries to DELETE organization -> 403
    let req = test::TestRequest::delete()
        .uri(&format!("/organizations/{}", org.id))
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);

    // 2. Owner tries to DELETE organization -> Should pass 200 (or 204/etc)
    let req = test::TestRequest::delete()
        .uri(&format!("/organizations/{}", org.id))
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Owner should be able to delete organization");
}

#[actix_web::test]
async fn test_course_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    
    // Setup Data
    let mut conn = setup_conn();
    let teacher = create_test_user(&mut conn, "teacher");
    let student = create_test_user(&mut conn, "student");
    
    let new_course = NewCourse { title: unique_string("TestCourse") };
    let course = diesel::insert_into(courses::table).values(&new_course).get_result::<Course>(&mut conn).unwrap();

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER");
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT");

    let teacher_token = generate_token(teacher.id());
    let student_token = generate_token(student.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope())
    ).await;

    // 1. Student tries to UPDATE course -> 403 (MODIFY_COURSE required)
    // Assuming STUDENT does NOT have MODIFY_COURSE.
    let req = test::TestRequest::put()
        .uri(&format!("/courses/{}", course.id))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .set_json(serde_json::json!({ "title": "Hacked Title" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);

    // 2. Teacher tries to UPDATE course -> Should pass (200)
    // Assuming TEACHER has MODIFY_COURSE.
    let req = test::TestRequest::put()
        .uri(&format!("/courses/{}", course.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "title": "Updated Title" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Teacher should be able to update course");
}
