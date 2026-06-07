use anyhow::{anyhow, Result};
use dotenvy::dotenv;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Semaphore;

use rust_learn::utils::worker as worker_utils;

/// Worker entrypoint. Uses a tokio Semaphore to limit the number of
/// concurrent ffmpeg processing tasks (controlled via WORKER_CONCURRENCY).
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize environment, DB pool, S3 and notifications state
    dotenv().ok();
    rust_learn::utils::logging::init_logging("worker");
    let pool = match rust_learn::db::try_establish_connection() {
        Ok(pool) => pool,
        Err(error) => {
            log::error!("event=worker_db_pool_init_failed error={}", error);
            return Err(anyhow!(error));
        }
    };

    let s3 = match rust_learn::utils::s3_utils::S3State::new_from_env().await {
        Ok(m) => m,
        Err(e) => {
            log::error!("event=worker_s3_init_failed error={:?}", e);
            return Err(e);
        }
    };

    let notifications = rust_learn::utils::notifications::NotificationsState::new(pool.clone());

    // Create shutdown flag and signal handler
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_handle = shutdown.clone();
    // spawn a task to listen for SIGTERM and SIGINT
    tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).ok();

        tokio::select! {
            _ = async {
                if let Some(sigterm) = sigterm.as_mut() {
                    let _ = sigterm.recv().await;
                } else {
                    std::future::pending::<()>().await;
                }
            } => {
                log::info!("event=worker_shutdown_signal signal=terminate action=stop_claiming");
            }
            result = tokio::signal::ctrl_c() => {
                if let Err(err) = result {
                    log::warn!(
                        "event=worker_shutdown_signal_listen_failed signal=interrupt error={:?}",
                        err
                    );
                }
                log::info!("event=worker_shutdown_signal signal=interrupt action=stop_claiming");
            }
        }

        log::info!("event=worker_shutdown_signal action=stop_claiming");
        shutdown_handle.store(true, Ordering::SeqCst);
    });

    // determine concurrency from env (default = 1)
    let concurrency: usize = worker_utils::positive_usize_from_env_value(
        std::env::var("WORKER_CONCURRENCY").ok().as_deref(),
        1,
    );

    let sem = Arc::new(Semaphore::new(concurrency));
    let deposit_indexer_handle =
        rust_learn::services::wallet_deposit_indexer_service::spawn_wallet_deposit_indexer(
            pool.clone(),
            shutdown.clone(),
        );

    // Write an initial alive stamp for healthcheck
    let _ = worker_utils::write_heartbeat(worker_utils::DEFAULT_WORKER_HEARTBEAT_PATH).await;

    // Configure retry/backoff behaviour
    let max_attempts: i64 = worker_utils::positive_i64_from_env_value(
        std::env::var("WORKER_MAX_ATTEMPTS").ok().as_deref(),
        5,
    );
    let base_backoff_seconds: u64 = worker_utils::positive_u64_from_env_value(
        std::env::var("WORKER_BASE_BACKOFF_SECONDS").ok().as_deref(),
        60,
    );

    loop {
        // if shutdown requested, stop claiming new jobs
        if shutdown.load(Ordering::SeqCst) {
            log::info!("event=worker_shutdown_requested action=wait_for_in_flight");
            break;
        }
        // Try to atomically claim a job and return it
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                log::error!(
                    "event=worker_db_connection_failed phase=claim error={:?}",
                    e
                );
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        // Use a transaction + FOR UPDATE SKIP LOCKED to safely claim a job without raw SQL
        // Select only queued jobs whose updated_at (used as available_at for retries)
        // is either NULL or <= now() so backoff delays are respected.

        // Stamp alive for healthcheck
        let _ = worker_utils::write_heartbeat(worker_utils::DEFAULT_WORKER_HEARTBEAT_PATH).await;

        match rust_learn::models::upload_job::UploadJob::queue_metrics(&mut conn).await {
            Ok(metrics) => {
                let in_flight = concurrency.saturating_sub(sem.available_permits());
                log::info!(
                    "event=worker_queue_metrics queue_depth={} queued_ready={} queued_delayed={} processing={} failed={} in_flight={} concurrency={}",
                    metrics.queue_depth(),
                    metrics.queued_ready,
                    metrics.queued_delayed,
                    metrics.processing,
                    metrics.failed,
                    in_flight,
                    concurrency
                );
            }
            Err(e) => {
                log::warn!("event=worker_queue_metrics_failed error={:?}", e);
            }
        }

        let job_opt: Option<rust_learn::models::upload_job::UploadJob> =
            match rust_learn::models::upload_job::UploadJob::claim_job(&mut conn).await {
                Ok(j) => j,
                Err(e) => {
                    log::error!("event=worker_job_claim_failed error={:?}", e);
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
                log::info!("event=worker_semaphore_closed action=exit");
                return Ok(());
            }
        };

        // Clone handles for the spawned task
        let s3_cloned = s3.clone();
        let notifications_cloned = notifications.clone();
        let mut conn_for_task = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                log::error!("event=worker_db_connection_failed phase=task error={:?}", e);
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
        let current_attempts = job.attempts as i64;
        let attempt_number = current_attempts + 1;

        log::info!(
            "event=worker_job_claimed job_id={} bucket={} object={} attempt={} previous_attempts={} max_attempts={}",
            job_id,
            bucket,
            object,
            attempt_number,
            current_attempts,
            max_attempts
        );

        // Spawn a detached task to process the job so loop can continue claiming jobs
        tokio::spawn(async move {
            let started_at = Instant::now();
            log::info!(
                "event=worker_job_started job_id={} attempt={} max_attempts={}",
                job_id,
                attempt_number,
                max_attempts
            );

            // Run the processing (use 0 for missing user_id handling inside process_uploaded_video if needed)
            let uid = user_id.unwrap_or(0);
            let notifications_for_processing = notifications_cloned.clone();
            let res = s3_cloned
                .process_uploaded_video(&bucket, &object, uid, notifications_for_processing)
                .await;
            let duration_ms = started_at.elapsed().as_millis();

            match res {
                Ok(()) => {
                    if let Err(e) = rust_learn::models::upload_job::UploadJob::mark_done(
                        job_id,
                        &mut conn_for_task,
                    )
                    .await
                    {
                        log::error!(
                            "event=worker_job_mark_done_failed job_id={} error={:?}",
                            job_id,
                            e
                        );
                    }
                    log::info!(
                        "event=worker_job_processed job_id={} result=done duration_ms={} attempt={} previous_attempts={}",
                        job_id,
                        duration_ms,
                        attempt_number,
                        current_attempts
                    );
                }
                Err(err) => {
                    let err_text = err.to_string();
                    let new_attempts = current_attempts + 1;
                    log::warn!(
                        "event=worker_job_processing_failed job_id={} bucket={} object={} duration_ms={} attempts={} max_attempts={} error={}",
                        job_id,
                        bucket,
                        object,
                        duration_ms,
                        new_attempts,
                        max_attempts,
                        err_text
                    );

                    if new_attempts >= max_attempts {
                        if let Some(uid) = user_id {
                            if let Err(e) = notifications_cloned
                                .send_worker_failure_notification(
                                    uid,
                                    job_id,
                                    &object,
                                    new_attempts as i32,
                                    &err_text,
                                )
                                .await
                            {
                                log::warn!(
                                    "event=notification_send_failed kind=worker_failure job_id={} user_id={} error={:?}",
                                    job_id,
                                    uid,
                                    e
                                );
                            }
                        }

                        // mark as permanently failed
                        if let Err(e) = rust_learn::models::upload_job::UploadJob::mark_failed(
                            job_id,
                            new_attempts as i32,
                            err_text.clone(),
                            &mut conn_for_task,
                        )
                        .await
                        {
                            log::error!(
                                "event=worker_job_mark_failed_failed job_id={} attempts={} error={:?}",
                                job_id,
                                new_attempts,
                                e
                            );
                        }
                        log::warn!(
                            "event=worker_job_terminal_failure job_id={} duration_ms={} attempts={} max_attempts={} failed_jobs_delta=1",
                            job_id,
                            duration_ms,
                            new_attempts,
                            max_attempts
                        );
                    } else {
                        // exponential backoff (base * 2^attempts)
                        // set updated_at to future time so claim SQL skips it until backoff expires
                        let future_time = worker_utils::retry_available_at(
                            chrono::Utc::now(),
                            base_backoff_seconds,
                            current_attempts,
                        );

                        if let Err(e) = rust_learn::models::upload_job::UploadJob::schedule_retry(
                            job_id,
                            new_attempts as i32,
                            err_text.clone(),
                            future_time,
                            &mut conn_for_task,
                        )
                        .await
                        {
                            log::error!(
                                "event=worker_job_schedule_retry_failed job_id={} attempts={} error={:?}",
                                job_id,
                                new_attempts,
                                e
                            );
                        }
                        log::info!(
                            "event=worker_job_retry_scheduled job_id={} duration_ms={} attempts={} max_attempts={} retry_available_at={}",
                            job_id,
                            duration_ms,
                            new_attempts,
                            max_attempts,
                            future_time
                        );
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
            log::info!("event=worker_exit reason=in_flight_finished");
            break;
        }
        log::info!(
            "event=worker_waiting_for_in_flight remaining_tasks={}",
            concurrency - available
        );
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    if let Some(handle) = deposit_indexer_handle {
        if let Err(error) = handle.await {
            log::warn!("event=wallet_deposit_indexer_join_failed error={:?}", error);
        }
    }

    Ok(())
}
