use actix_web::{test, App, web};
use diesel::prelude::*;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::models::user::User;
use rust_learn::utils::jwt_utils::create_jwt;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::course::{NewCourse, Course};
use rust_learn::db::schema::{organizations, courses};
use rust_learn::models::role::{PlatformRole, OrganizationRole, CourseRole};
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_course::UserRoleCourse;
use chrono::NaiveDate;
use actix_service::Service; // Import Service trait for .call()

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(pool: &DbPool) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get().await.expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
    let email = unique_string(name) + "@example.com";
    create_user(
        conn,
        name,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password",
    )
    .await
    .expect("failed to create user")
}

fn generate_token(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to generate token")
}

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn).await.expect("role not found");
    UserRolePlatform::assign(conn, user_id, role_id).await.expect("assign failed");
}

async fn force_assign_org_role(conn: &mut AsyncPgConnection, user_id: i32, org_id: i32, role_name: &str) {
    let role_id = OrganizationRole::find_by_name(role_name, conn).await.expect("role not found");
    UserRoleOrganization::assign(conn, user_id, org_id, role_id).await.expect("assign failed");
}

async fn force_assign_course_role(conn: &mut AsyncPgConnection, user_id: i32, course_id: i32, role_name: &str) {
    let role_id = CourseRole::find_by_name(role_name, conn).await.expect("role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id).await.expect("assign failed");
}

#[actix_web::test]
async fn test_platform_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    // Setup Users
    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "superadmin").await;
    let target_user = create_test_user(&mut conn, "target").await;
    let unprivileged = create_test_user(&mut conn, "unpriv").await;

    // Assign SUPER_ADMIN role (which has all perms)
    force_assign_platform_role(&mut conn, admin.id(), "SUPER_ADMIN").await;

    let admin_token = generate_token(admin.id());
    let unprivileged_token = generate_token(unprivileged.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::users::user_scope())
    ).await;

    // 1. Unprivileged user tries to assign role -> Should Fail
    let req = test::TestRequest::post()
        .uri(&format!("/user/{}/role", target_user.id()))
        .insert_header(("Authorization", format!("Bearer {}", unprivileged_token)))
        .set_json(serde_json::json!({ "role_name": "STUDENT" }))
        .to_request();
    
    // middleware returns Err, so app.call() returns Err
    let result = app.call(req).await;
    match result {
        Ok(resp) => {
            // It might pass if logic changes, but we expect error or 403
            if resp.status().is_success() {
                 panic!("Unprivileged user access succeeded unexpectedly");
            }
            assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
        },
        Err(e) => {
             // Middleware error is returned here
             let resp = e.error_response();
             assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 2. Admin user tries to assign role -> Should Succeed or pass middleware
    let req = test::TestRequest::post()
        .uri(&format!("/user/{}/role", target_user.id()))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(serde_json::json!({ "role_name": "USER" })) 
        .to_request();

    let result = app.call(req).await;
    match result {
        Ok(resp) => {
             // Admin > User, so assignment might succeed (200) or fail on logic details, but OK is expected
             assert!(resp.status().is_success(), "Admin request failed: status {}", resp.status());
        },
        Err(e) => {
             panic!("Admin request returned error: {}", e);
        }
    }
}

#[actix_web::test]
async fn test_organization_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    
    // Setup Data
    let mut conn = setup_conn(&pool).await;
    let owner = create_test_user(&mut conn, "org_superadmin").await;
    let stranger = create_test_user(&mut conn, "stranger").await;
    
    let new_org = NewOrganization { name: unique_string("TestOrg"), website_link: None, profile_url: None };
    let org = diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result::<Organization>(&mut conn)
        .await
        .unwrap();

    // Assign ADMIN (Org scope) - Note: SUPERADMIN has permission sync issues in current migrations
    force_assign_org_role(&mut conn, owner.id(), org.id, "ADMIN").await;

    let owner_token = generate_token(owner.id());
    let stranger_token = generate_token(stranger.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope())
    ).await;

    // 1. Stranger (no role) tries to UPDATE organization -> 403
    let req = test::TestRequest::put()
        .uri(&format!("/organizations/{}", org.id))
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .set_json(serde_json::json!({ "name": "Hacked Org" }))
        .to_request();
    
    let result = app.call(req).await;
    match result {
        Ok(resp) => assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN),
        Err(e) => {
            let resp = e.error_response();
            assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 2. Owner tries to UPDATE organization -> Should pass
    let req = test::TestRequest::put()
        .uri(&format!("/organizations/{}", org.id))
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(serde_json::json!({ "name": "Updated Org" }))
        .to_request();

    let result = app.call(req).await;
     match result {
        Ok(resp) => assert!(resp.status().is_success(), "Owner request failed"),
        Err(e) => panic!("Owner request returned error: {}", e),
    }
}

#[actix_web::test]
async fn test_course_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    
    // Setup Data
    let mut conn = setup_conn(&pool).await;
    let teacher = create_test_user(&mut conn, "teacher").await;
    let student = create_test_user(&mut conn, "student").await;
    
    let new_course = NewCourse { title: unique_string("TestCourse") };
    let course = diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result::<Course>(&mut conn)
        .await
        .unwrap();

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;

    let teacher_token = generate_token(teacher.id());
    let student_token = generate_token(student.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope())
    ).await;

    // 1. Student tries to UPDATE course -> 403 (MANAGE_COURSE_SETTINGS required)
    // STUDENT does NOT have MANAGE_COURSE_SETTINGS.
    let req = test::TestRequest::put()
        .uri(&format!("/courses/{}", course.id))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .set_json(serde_json::json!({ "title": "Hacked Title" }))
        .to_request();
    
    let result = app.call(req).await;
    match result {
        Ok(resp) => assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN),
        Err(e) => {
             let resp = e.error_response();
             assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 2. Teacher tries to UPDATE course -> Should pass (200)
    // TEACHER has MANAGE_COURSE_SETTINGS.
    let req = test::TestRequest::put()
        .uri(&format!("/courses/{}", course.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "title": "Updated Title" }))
        .to_request();
    
    let result = app.call(req).await;
     match result {
        Ok(resp) => assert!(resp.status().is_success(), "Teacher request failed"),
        Err(e) => panic!("Teacher request returned error: {}", e),
    }
}
