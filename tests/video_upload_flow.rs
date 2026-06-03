use actix_service::Service;
use actix_web::{test, web, App};
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{courses, notifications, upload_jobs};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::chapter::Chapter;
use rust_learn::models::content::Content;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::role::CourseRole;
use rust_learn::models::upload_job::UploadJob;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use rust_learn::utils::notifications::NotificationsState;
use rust_learn::utils::s3_utils::S3State;
use serde::Deserialize;
use std::path::Path;
use std::process::Command;
use std::time::Duration as StdDuration;

#[derive(Deserialize)]
struct UploadUrlResponse {
    upload_url: String,
    object_key: String,
}

fn unique_string(prefix: &str) -> String {
    let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
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

fn generate_sample_video(path: &Path) {
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-hide_banner",
            "-loglevel",
            "error",
            "-f",
            "lavfi",
            "-i",
            "testsrc=duration=1:size=160x90:rate=10",
            "-f",
            "lavfi",
            "-i",
            "sine=duration=1:frequency=440:sample_rate=44100",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "aac",
            "-shortest",
            path.to_string_lossy().as_ref(),
        ])
        .status()
        .expect("ffmpeg must be installed on PATH to run the video upload flow test");

    assert!(status.success(), "ffmpeg failed to generate sample video");
}

async fn assert_s3_object_downloadable(s3: &S3State, object: &str) {
    let url = s3
        .presign_get("course-materials", object, 60)
        .await
        .expect("processed object should be presignable");
    let response = reqwest::get(url)
        .await
        .expect("processed object should be reachable");
    let status = response.status();
    let bytes = response.bytes().await.expect("response bytes should load");

    assert!(
        status.is_success(),
        "processed object {} should download successfully, got {}",
        object,
        status
    );
    assert!(!bytes.is_empty(), "processed object {} is empty", object);
}

async fn wait_for_running_worker(
    conn: &mut AsyncPgConnection,
    job_id: i64,
    timeout: StdDuration,
) -> UploadJob {
    let started = std::time::Instant::now();

    loop {
        let job = upload_jobs::table
            .find(job_id)
            .first::<UploadJob>(conn)
            .await
            .expect("upload job should remain queryable");

        match job.status.as_str() {
            "done" => return job,
            "failed" => panic!(
                "running worker failed upload job {} after {} attempts: {:?}",
                job.id(),
                job.attempts,
                job.last_error
            ),
            _ if started.elapsed() >= timeout => panic!(
                "timed out waiting for running worker to process job {}; last status: {}",
                job.id(),
                job.status
            ),
            _ => tokio::time::sleep(StdDuration::from_millis(500)).await,
        }
    }
}

#[actix_web::test]
async fn course_video_upload_can_be_queued_and_processed() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let s3 = S3State::new_from_env().await.expect("init s3");
    s3.health_check()
        .await
        .expect("S3 must be reachable through S3_INTERNAL_* to run this test");
    let notifications = NotificationsState::new(pool.clone());

    let mut conn = setup_conn(&pool).await;
    let teacher = create_test_user(&mut conn, "teacher_video_upload").await;
    let course = diesel::insert_into(courses::table)
        .values(&NewCourse {
            title: unique_string("VideoUploadCourse"),
        })
        .get_result::<Course>(&mut conn)
        .await
        .expect("course should be created");
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let teacher_token = create_jwt(teacher.id()).expect("failed to generate token");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(s3.clone()))
            .app_data(web::Data::new(notifications.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/courses/{}/chapters", course.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "title": "Video chapter",
            "order": 1
        }))
        .to_request();
    let resp = app.call(req).await.expect("chapter request should run");
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    let chapter: Chapter = test::read_body_json(resp).await;

    let sample_path = std::env::temp_dir().join(format!("{}.mp4", unique_string("sample_video")));
    generate_sample_video(&sample_path);
    let filename = sample_path
        .file_name()
        .expect("sample video path should have filename")
        .to_string_lossy()
        .to_string();

    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents/upload_url",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "filename": filename,
            "content_type": "video/mp4"
        }))
        .to_request();
    let resp = app.call(req).await.expect("upload URL request should run");
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    let upload: UploadUrlResponse = test::read_body_json(resp).await;

    let sample_bytes = tokio::fs::read(&sample_path)
        .await
        .expect("sample video should be readable");
    let upload_response = reqwest::Client::new()
        .put(&upload.upload_url)
        .body(sample_bytes)
        .send()
        .await
        .expect("sample video PUT should reach object storage");
    let upload_status = upload_response.status();
    let upload_body = upload_response.text().await.unwrap_or_default();
    assert!(
        upload_status.is_success(),
        "sample video upload failed with {}: {}",
        upload_status,
        upload_body
    );

    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "order": 1,
            "content_type": "video",
            "data": upload.object_key
        }))
        .to_request();
    let resp = app
        .call(req)
        .await
        .expect("content create request should run");
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    let content: Content = test::read_body_json(resp).await;
    assert_eq!(content.content_type, "video");
    assert_eq!(content.data.as_deref(), Some(upload.object_key.as_str()));

    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents/{}/process",
            course.id, chapter.id, content.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .to_request();
    let resp = app.call(req).await.expect("process request should run");
    assert_eq!(resp.status(), actix_web::http::StatusCode::ACCEPTED);

    let mut conn = setup_conn(&pool).await;
    let job = upload_jobs::table
        .filter(upload_jobs::object.eq(&upload.object_key))
        .order(upload_jobs::id.desc())
        .first::<UploadJob>(&mut conn)
        .await
        .expect("processing endpoint should create an upload job");
    assert_eq!(job.bucket, "course-materials");
    assert_eq!(job.user_id, Some(teacher.id()));

    let use_running_worker = std::env::var("VIDEO_UPLOAD_TEST_USE_RUNNING_WORKER")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if use_running_worker {
        wait_for_running_worker(&mut conn, job.id(), StdDuration::from_secs(60)).await;
    } else {
        assert_eq!(job.status, "queued");
        diesel::update(upload_jobs::table.find(job.id()))
            .set((
                upload_jobs::status.eq("processing"),
                upload_jobs::updated_at.eq(Utc::now()),
            ))
            .execute(&mut conn)
            .await
            .expect("test should mark the job as claimed by the worker");

        s3.process_uploaded_video(
            "course-materials",
            &upload.object_key,
            teacher.id(),
            notifications.clone(),
        )
        .await
        .expect("worker video processing should succeed");
        UploadJob::mark_done(job.id(), &mut conn)
            .await
            .expect("processed job should be marked done");

        let updated_job = upload_jobs::table
            .find(job.id())
            .first::<UploadJob>(&mut conn)
            .await
            .expect("processed job should still exist");
        assert_eq!(updated_job.status, "done");
    }

    let processed_video = format!("processed/{}", upload.object_key);
    let processed_audio = format!(
        "processed/audio/{}.mp3",
        upload.object_key.replace('/', "_")
    );
    assert_s3_object_downloadable(&s3, &processed_video).await;
    assert_s3_object_downloadable(&s3, &processed_audio).await;

    let processed_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(teacher.id())))
        .filter(notifications::title.eq("video:processed"))
        .filter(notifications::body.like(format!("%{}%", upload.object_key)))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("processed notification query should succeed");
    assert!(processed_notification_count >= 1);

    let _ = tokio::fs::remove_file(sample_path).await;
}
