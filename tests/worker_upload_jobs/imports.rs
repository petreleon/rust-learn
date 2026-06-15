use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::upload_jobs;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::infra::postgres::content::upload_job_queue;
use rust_learn::models::upload_job::{NewUploadJob, UploadJob};
use std::sync::LazyLock;
use tokio::sync::{Mutex, MutexGuard};

static WORKER_UPLOAD_JOB_TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

async fn lock_worker_upload_job_tests() -> MutexGuard<'static, ()> {
    WORKER_UPLOAD_JOB_TEST_LOCK.lock().await
}

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
    let _guard = lock_worker_upload_job_tests().await;
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    conn.transaction::<(), diesel::result::Error, _>(|tx| {
        Box::pin(async move {
            let before = upload_job_queue::queue_metrics(tx)
                .await
                .expect("queue metrics should load before seeded jobs");

            let ready = insert_upload_job(tx, &unique_object("metrics-ready")).await;
            let delayed = insert_upload_job(tx, &unique_object("metrics-delayed")).await;
            let processing = insert_upload_job(tx, &unique_object("metrics-processing")).await;
            let failed = insert_upload_job(tx, &unique_object("metrics-failed")).await;

            diesel::update(upload_jobs::table.find(delayed.id()))
                .set(upload_jobs::updated_at.eq(Utc::now() + Duration::minutes(10)))
                .execute(tx)
                .await
                .expect("test should delay a queued job");
            diesel::update(upload_jobs::table.find(processing.id()))
                .set(upload_jobs::status.eq("processing"))
                .execute(tx)
                .await
                .expect("test should mark a job processing");
            upload_job_queue::mark_failed(failed.id(), 3, "metrics failure".to_string(), tx)
                .await
                .expect("test should mark a job failed");

            let after = upload_job_queue::queue_metrics(tx)
                .await
                .expect("queue metrics should load after seeded jobs");

            assert_eq!(after.queued_ready, before.queued_ready + 1);
            assert_eq!(after.queued_delayed, before.queued_delayed + 1);
            assert_eq!(after.processing, before.processing + 1);
            assert_eq!(after.failed, before.failed + 1);
            assert_eq!(after.queue_depth(), before.queue_depth() + 2);

            for job in [ready, delayed, processing, failed] {
                upload_job_queue::mark_done(job.id(), tx)
                    .await
                    .expect("test should clean up metrics job");
            }

            Ok(())
        })
    })
    .await
    .expect("metrics transaction should commit");
}
