use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use anyhow::Result;
use dotenvy::dotenv;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::query_dsl::methods::LockingDsl;
// DbPool import not needed in this binary
use tokio::sync::Semaphore;
use tokio::fs as tokio_fs;
use tokio::signal::unix::{signal, SignalKind};

/// Worker entrypoint. Uses a tokio Semaphore to limit the number of
/// concurrent ffmpeg processing tasks (controlled via WORKER_CONCURRENCY).
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize environment, DB pool, MinIO and notifications state
    dotenv().ok();
    let pool = rust_learn::db::establish_connection();

    let minio = match rust_learn::utils::minio_utils::MinioState::new_from_env().await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to init MinIO: {:?}", e);
            return Err(e);
        }
    };

    let notifications = rust_learn::utils::notifications::NotificationsState::new(pool.clone());

    // Create shutdown flag and signal handler
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_handle = shutdown.clone();
    // spawn a task to listen for SIGTERM and SIGINT
    tokio::spawn(async move {
        // Listen for SIGTERM
        if let Ok(mut sigterm) = signal(SignalKind::terminate()) {
            let _ = sigterm.recv().await;
        }
        // Also listen for Ctrl-C as fallback
        let _ = tokio::signal::ctrl_c().await;
        eprintln!("Received shutdown signal, stopping worker claims...");
        shutdown_handle.store(true, Ordering::SeqCst);
    });

    // determine concurrency from env (default = 1)
    let concurrency: usize = std::env::var("WORKER_CONCURRENCY").ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(1);

    let sem = Arc::new(Semaphore::new(concurrency));

    // Write an initial alive stamp for healthcheck
    let _ = tokio_fs::write("/tmp/worker_alive", format!("{}", chrono::Utc::now().timestamp())).await;

    // Configure retry/backoff behaviour
    let max_attempts: i64 = std::env::var("WORKER_MAX_ATTEMPTS").ok()
        .and_then(|s| s.parse::<i64>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(5);
    let base_backoff_seconds: u64 = std::env::var("WORKER_BASE_BACKOFF_SECONDS").ok()
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(60);

    loop {
        // if shutdown requested, stop claiming new jobs
        if shutdown.load(Ordering::SeqCst) {
            eprintln!("Shutdown requested: stopping job claims and waiting for in-flight tasks to finish");
            break;
        }
        // Try to atomically claim a job and return it
        let mut conn = match pool.get() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to get DB connection: {:?}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        // Use a transaction + FOR UPDATE SKIP LOCKED to safely claim a job without raw SQL
        // Select only queued jobs whose updated_at (used as available_at for retries)
        // is either NULL or <= now() so backoff delays are respected.


        // Stamp alive for healthcheck
        let _ = tokio_fs::write("/tmp/worker_alive", format!("{}", chrono::Utc::now().timestamp())).await;

        let job_opt: Option<rust_learn::models::upload_job::UploadJob> = match rust_learn::models::upload_job::UploadJob::claim_job(&mut conn) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Failed to claim job: {:?}", e);
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
        };

        let job = match job_opt {
            Some(j) => j,
            None => {
                // No queued jobs; sleep and retry
                tokio::time::sleep(Duration::from_secs(3)).await;
                continue;
            }
        };

        // Acquire a permit before spawning the task so we don't exceed concurrency
        let permit = match sem.clone().acquire_owned().await {
            Ok(p) => p,
            Err(_) => {
                // semaphore closed; graceful exit
                eprintln!("Semaphore closed, exiting worker loop");
                return Ok(());
            }
        };

        // Clone handles for the spawned task
        let minio_cloned = minio.clone();
        let notifications_cloned = notifications.clone();
        let mut conn_for_task = match pool.get() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to get DB connection for task: {:?}", e);
                // release permit by dropping it and continue
                drop(permit);
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
        };

        let job_id = job.id;
        let bucket = job.bucket.clone();
        let object = job.object.clone();
        let user_id = job.user_id;

    // Spawn a detached task to process the job so loop can continue claiming jobs
        tokio::spawn(async move {
            // Run the processing (use 0 for missing user_id handling inside process_uploaded_video if needed)
            let uid = user_id.unwrap_or(0);
            let res = minio_cloned.process_uploaded_video(&bucket, &object, uid, notifications_cloned).await;

            if res.is_ok() {
                if let Err(e) = rust_learn::models::upload_job::UploadJob::mark_done(job_id, &mut *conn_for_task) {
                    eprintln!("Failed to mark job done {}: {:?}", job_id, e);
                }
            } else {
                let err_text = format!("{}", res.err().unwrap());
                let current_attempts = job.attempts as i64;
                let new_attempts = current_attempts + 1;

                if new_attempts >= max_attempts {
                    // mark as permanently failed
                    if let Err(e) = rust_learn::models::upload_job::UploadJob::mark_failed(job_id, new_attempts as i32, err_text.clone(), &mut *conn_for_task) {
                        eprintln!("Failed to mark job failed {}: {:?}", job_id, e);
                    }
                } else {
                    // exponential backoff (base * 2^attempts)
                    let backoff = base_backoff_seconds.saturating_mul(2u64.saturating_pow(current_attempts as u32));
                    // set updated_at to future time so claim SQL skips it until backoff expires
                    let future_time = chrono::Utc::now()
                        .checked_add_signed(chrono::Duration::seconds(backoff as i64))
                        .unwrap_or_else(chrono::Utc::now);

                    if let Err(e) = rust_learn::models::upload_job::UploadJob::schedule_retry(job_id, new_attempts as i32, err_text.clone(), future_time, &mut *conn_for_task) {
                        eprintln!("Failed to schedule retry for job {}: {:?}", job_id, e);
                    }
                }
            }

            // permit is dropped here when going out of scope, releasing it
            drop(permit);
        });
    }

    // Wait for in-flight tasks to finish: available_permits == concurrency
    loop {
        let available = sem.available_permits();
        if available >= concurrency {
            eprintln!("All in-flight tasks finished, worker exiting");
            break;
        }
        eprintln!("Waiting for {} in-flight tasks to finish...", concurrency - available);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
