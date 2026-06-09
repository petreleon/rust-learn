use actix_service::Service;
use actix_web::{test, web, App};
use chrono::NaiveDate;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{chapters, courses, upload_jobs};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::chapter::{Chapter, NewChapter};
use rust_learn::models::content::Content;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::role::CourseRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
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

async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("assign failed");
}

#[actix_web::test]
async fn test_course_content_lifecycle() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    // Setup Data
    let mut conn = setup_conn(&pool).await;
    let teacher = create_test_user(&mut conn, "teacher_content").await;
    let student = create_test_user(&mut conn, "student_content").await;
    let outsider = create_test_user(&mut conn, "outsider_content").await;

    let new_course = NewCourse {
        title: unique_string("CourseWithContent"),
    };
    let course = diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result::<Course>(&mut conn)
        .await
        .unwrap();
    let other_course = diesel::insert_into(courses::table)
        .values(&NewCourse {
            title: unique_string("OtherContentCourse"),
        })
        .get_result::<Course>(&mut conn)
        .await
        .unwrap();
    let other_chapter = diesel::insert_into(chapters::table)
        .values(NewChapter {
            course_id: other_course.id,
            title: "Other course chapter".to_string(),
            order: 1,
        })
        .get_result::<Chapter>(&mut conn)
        .await
        .unwrap();

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;

    let teacher_token = generate_token(teacher.id());
    let student_token = generate_token(student.id());
    let outsider_token = generate_token(outsider.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Mock S3 state or rely on it failing gracefully if not needed for metadata tests?
            // But app_scope tries to inject it in main.rs. Here we are initializing specific scopes.
            // But app_scope tries to inject it in main.rs. Here we are initializing specific scopes.
            // We need to mirror api_scope's usage of scopes.
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    // 1. Teacher CREATES Chapter (/courses/{id}/chapters)
    let req = test::TestRequest::post()
        .uri(&format!("/courses/{}/chapters", course.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "title": "Intro", "order": 1 }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(
        resp.status().is_success(),
        "Create Chapter failed: {}",
        resp.status()
    );

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
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "order": 1,
            "content_type": "text",
            "data": "Welcome to the course"
        }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(
        resp.status().is_success(),
        "Create Content failed: {}",
        resp.status()
    );
    let content: Content = test::read_body_json(resp).await;
    assert_eq!(content.data.unwrap(), "Welcome to the course");

    // 3b. Teacher cannot use their course permission with a chapter from another course.
    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, other_chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "order": 1,
            "content_type": "text",
            "data": "Cross-course write"
        }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);

    // 4. Student lists content through VIEW_CONTENT
    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success());
    let listed_content: Vec<Content> = test::read_body_json(resp).await;
    assert_eq!(listed_content.len(), 1);

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, other_chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);

    // 5. User without course content permission cannot list content
    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", outsider_token)))
        .to_request();
    let resp = app.call(req).await;
    match resp {
        Ok(r) => {
            if r.status().is_success() {
                panic!("Outsider listed content!");
            }
            assert_eq!(r.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
        Err(e) => {
            let r = e.error_response();
            assert_eq!(r.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 6. Student Cannot Create Content -> 403
    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
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
            if r.status().is_success() {
                panic!("Student created content!");
            }
            assert_eq!(r.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
        Err(e) => {
            let r = e.error_response();
            assert_eq!(r.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 7. Update Content (Teacher)
    let req = test::TestRequest::put()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents/{}",
            course.id, chapter.id, content.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "data": "Updated Text" }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success());
    let updated_content: Content = test::read_body_json(resp).await;
    assert_eq!(updated_content.data.unwrap(), "Updated Text");

    let mut conn = setup_conn(&pool).await;
    let queued_text_jobs_before = upload_jobs::table
        .filter(upload_jobs::object.eq("Updated Text"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("upload job count query should succeed");
    drop(conn);

    // 8. Teacher cannot trigger video processing for text content
    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents/{}/process",
            course.id, chapter.id, content.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    let mut conn = setup_conn(&pool).await;
    let queued_text_jobs_after = upload_jobs::table
        .filter(upload_jobs::object.eq("Updated Text"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("upload job count query should succeed");
    assert_eq!(queued_text_jobs_after, queued_text_jobs_before);
}
