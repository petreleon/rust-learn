use actix_web::{test, App, web};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::utils::db_utils::authentication_registration::create_user;
use rust_learn::models::user::User;
use rust_learn::utils::jwt_utils::create_jwt;
use rust_learn::models::course::{NewCourse, Course};
use rust_learn::db::schema::{courses, chapters, contents};
use rust_learn::models::chapter::{Chapter, NewChapter};
use rust_learn::models::content::{Content, NewContent};
use rust_learn::models::role::CourseRole;
use rust_learn::models::user_role_course::UserRoleCourse;
use chrono::NaiveDate;
use actix_service::Service;

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
    create_jwt(user_id).expect("failed to generate token")
}

fn force_assign_course_role(conn: &mut PgConnection, user_id: i32, course_id: i32, role_name: &str) {
    let role_id = CourseRole::find_by_name(role_name, conn).expect("role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id).expect("assign failed");
}

#[actix_web::test]
async fn test_course_content_lifecycle() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    
    // Setup Data
    let mut conn = setup_conn();
    let teacher = create_test_user(&mut conn, "teacher_content");
    let student = create_test_user(&mut conn, "student_content");
    
    let new_course = NewCourse { title: unique_string("CourseWithContent") };
    let course = diesel::insert_into(courses::table).values(&new_course).get_result::<Course>(&mut conn).unwrap();

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER");
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT");

    let teacher_token = generate_token(teacher.id());
    let student_token = generate_token(student.id());

    let app = test::init_service(
         App::new()
            .app_data(web::Data::new(pool.clone()))
             // Mock MinIO state or rely on it failing gracefully if not needed for metadata tests?
             // Since upload creates separate endpoint, strictly metadata operations don't need MinIO.
             // But app_scope tries to inject it in main.rs. Here we are initializing specific scopes.
             // We need to mirror api_scope's usage of scopes.
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope())
    ).await;

    // 1. Teacher CREATES Chapter (/courses/{id}/chapters)
    let req = test::TestRequest::post()
        .uri(&format!("/courses/{}/chapters", course.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "title": "Intro", "order": 1 }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success(), "Create Chapter failed: {}", resp.status());
    
    let chapter: Chapter = test::read_body_json(resp).await;
    assert_eq!(chapter.title, "Intro");

    // 2. Student List Chapters (/courses/{id}/chapters)
    let req = test::TestRequest::get()
        .uri(&format!("/courses/{}/chapters", course.id))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success());
    let chapters: Vec<Chapter> = test::read_body_json(resp).await;
    assert_eq!(chapters.len(), 1);

    // 3. Teacher CREATES Content (/courses/{id}/chapters/{cid}/contents)
    let req = test::TestRequest::post()
        .uri(&format!("/courses/{}/chapters/{}/contents", course.id, chapter.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ 
            "order": 1, 
            "content_type": "text", 
            "data": "Welcome to the course" 
        }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success(), "Create Content failed: {}", resp.status());
    let content: Content = test::read_body_json(resp).await;
    assert_eq!(content.data.unwrap(), "Welcome to the course");

     // 4. Student Cannot Create Content -> 403
    let req = test::TestRequest::post()
        .uri(&format!("/courses/{}/chapters/{}/contents", course.id, chapter.id))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .set_json(serde_json::json!({ 
            "order": 2, 
            "content_type": "video", 
            "data": "hack" 
        }))
        .to_request();
    // Use try_call check logic manually or app.call
    let resp = app.call(req).await;
    match resp {
        Ok(r) => {
             // Expect 403
             if r.status().is_success() { panic!("Student created content!"); }
             assert_eq!(r.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
        Err(e) => {
            let r = e.error_response();
            assert_eq!(r.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 5. Update Content (Teacher)
    let req = test::TestRequest::put()
        .uri(&format!("/courses/{}/chapters/{}/contents/{}", course.id, chapter.id, content.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "data": "Updated Text" }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success());
    let updated_content: Content = test::read_body_json(resp).await;
    assert_eq!(updated_content.data.unwrap(), "Updated Text");

    // 6. Teacher Triggers Processing
    let req = test::TestRequest::post()
        .uri(&format!("/courses/{}/chapters/{}/contents/{}/process", course.id, chapter.id, content.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), actix_web::http::StatusCode::ACCEPTED);
}
