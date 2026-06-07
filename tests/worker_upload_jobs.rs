use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::upload_jobs;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::upload_job::{NewUploadJob, UploadJob};

fn unique_object(prefix: &str) -> String {
    let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("tests/worker/{}-{}-{}.mp4", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn insert_upload_job(conn: &mut AsyncPgConnection, object: &str) -> UploadJob {
    let new_job = NewUploadJob {
        bucket: "worker-test-bucket",
        object,
        user_id: None,
    };

    diesel::insert_into(upload_jobs::table)
        .values(&new_job)
        .get_result(conn)
        .await
        .expect("upload job insert should succeed")
}

async fn fetch_upload_job(conn: &mut AsyncPgConnection, id: i64) -> UploadJob {
    upload_jobs::table
        .find(id)
        .first(conn)
        .await
        .expect("upload job should exist")
}

#[actix_web::test]
async fn queue_metrics_counts_ready_delayed_processing_and_failed_jobs() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let before = UploadJob::queue_metrics(&mut conn)
        .await
        .expect("queue metrics should load before seeded jobs");

    let ready = insert_upload_job(&mut conn, &unique_object("metrics-ready")).await;
    let delayed = insert_upload_job(&mut conn, &unique_object("metrics-delayed")).await;
    let processing = insert_upload_job(&mut conn, &unique_object("metrics-processing")).await;
    let failed = insert_upload_job(&mut conn, &unique_object("metrics-failed")).await;

    diesel::update(upload_jobs::table.find(delayed.id()))
        .set(upload_jobs::updated_at.eq(Utc::now() + Duration::minutes(10)))
        .execute(&mut conn)
        .await
        .expect("test should delay a queued job");
    diesel::update(upload_jobs::table.find(processing.id()))
        .set(upload_jobs::status.eq("processing"))
        .execute(&mut conn)
        .await
        .expect("test should mark a job processing");
    UploadJob::mark_failed(failed.id(), 3, "metrics failure".to_string(), &mut conn)
        .await
        .expect("test should mark a job failed");

    let after = UploadJob::queue_metrics(&mut conn)
        .await
        .expect("queue metrics should load after seeded jobs");

    assert!(after.queued_ready > before.queued_ready);
    assert!(after.queued_delayed > before.queued_delayed);
    assert!(after.processing > before.processing);
    assert!(after.failed > before.failed);
    assert!(after.queue_depth() >= before.queue_depth() + 2);

    UploadJob::mark_done(ready.id(), &mut conn)
        .await
        .expect("test should clean up ready job");
}

#[actix_web::test]
async fn schedule_retry_sets_queued_state_attempts_error_and_future_availability() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let job = insert_upload_job(&mut conn, &unique_object("retry")).await;

    diesel::update(upload_jobs::table.find(job.id()))
        .set(upload_jobs::status.eq("processing"))
        .execute(&mut conn)
        .await
        .expect("test should mark job processing");

    let retry_at = Utc::now() + Duration::minutes(5);
    UploadJob::schedule_retry(
        job.id(),
        1,
        "transient ffmpeg failure".to_string(),
        retry_at,
        &mut conn,
    )
    .await
    .expect("schedule_retry should succeed");

    let updated = fetch_upload_job(&mut conn, job.id()).await;
    assert_eq!(updated.status, "queued");
    assert_eq!(updated.attempts, 1);
    assert_eq!(
        updated.last_error.as_deref(),
        Some("transient ffmpeg failure")
    );
    let updated_at = updated.updated_at.expect("retry should set updated_at");
    assert!(updated_at > Utc::now());
    assert!(updated_at <= retry_at + Duration::seconds(1));
}

#[actix_web::test]
async fn mark_failed_sets_terminal_failure_state() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let job = insert_upload_job(&mut conn, &unique_object("failed")).await;

    UploadJob::mark_failed(
        job.id(),
        5,
        "permanent processing failure".to_string(),
        &mut conn,
    )
    .await
    .expect("mark_failed should succeed");

    let updated = fetch_upload_job(&mut conn, job.id()).await;
    assert_eq!(updated.status, "failed");
    assert_eq!(updated.attempts, 5);
    assert_eq!(
        updated.last_error.as_deref(),
        Some("permanent processing failure")
    );
    assert!(updated.updated_at.is_some());
}

#[actix_web::test]
async fn mark_done_sets_terminal_success_state_without_changing_attempts() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let job = insert_upload_job(&mut conn, &unique_object("done")).await;

    diesel::update(upload_jobs::table.find(job.id()))
        .set(upload_jobs::attempts.eq(2))
        .execute(&mut conn)
        .await
        .expect("test should seed attempts");

    UploadJob::mark_done(job.id(), &mut conn)
        .await
        .expect("mark_done should succeed");

    let updated = fetch_upload_job(&mut conn, job.id()).await;
    assert_eq!(updated.status, "done");
    assert_eq!(updated.attempts, 2);
    assert!(updated.updated_at.is_some());
}
