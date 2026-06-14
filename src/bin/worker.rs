#[path = "worker/failure.rs"]
mod failure;
#[path = "worker/jobs.rs"]
mod jobs;
#[path = "worker/metrics.rs"]
mod metrics;
#[path = "worker/runtime.rs"]
mod runtime;

use anyhow::Result;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Semaphore, TryAcquireError};

use jobs::{spawn_processing_task, RetryConfig};
use metrics::log_queue_metrics;

const WORKER_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const DEFAULT_WORKER_QUEUE_METRICS_INTERVAL: Duration = Duration::from_secs(60);

/// Worker entrypoint. Claims upload jobs and delegates processing to bounded tasks.
#[tokio::main]
async fn main() -> Result<()> {
    runtime::init_environment();
    let pool = runtime::init_pool()?;
    let s3 = runtime::init_s3().await?;
    let notifications = rust_learn::infra::notifications::NotificationsState::new(pool.clone());
    let shutdown = runtime::spawn_shutdown_listener();

    let concurrency = runtime::worker_concurrency();
    let sem = Arc::new(Semaphore::new(concurrency));
    let queue_metrics_interval =
        runtime::queue_metrics_interval(DEFAULT_WORKER_QUEUE_METRICS_INTERVAL);
    let mut last_queue_metrics_log: Option<Instant> = None;
    let retry_config = RetryConfig::from_env();

    let deposit_indexer_handle =
        rust_learn::infra::ethereum::wallet::deposit_indexer::spawn_wallet_deposit_indexer(
            pool.clone(),
            shutdown.clone(),
        );
    let heartbeat_handle = runtime::spawn_heartbeat(WORKER_HEARTBEAT_INTERVAL);

    loop {
        if shutdown.load(Ordering::SeqCst) {
            log::info!("event=worker_shutdown_requested action=wait_for_in_flight");
            break;
        }

        let permit = match sem.clone().try_acquire_owned() {
            Ok(permit) => permit,
            Err(TryAcquireError::NoPermits) => {
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
            Err(TryAcquireError::Closed) => {
                log::info!("event=worker_semaphore_closed action=exit");
                return Ok(());
            }
        };

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                log::error!(
                    "event=worker_db_connection_failed phase=claim error={:?}",
                    e
                );
                drop(permit);
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        let should_log_metrics = last_queue_metrics_log
            .map(|logged_at| logged_at.elapsed() >= queue_metrics_interval)
            .unwrap_or(true);
        if should_log_metrics {
            log_queue_metrics(&mut conn, &sem, concurrency, queue_metrics_interval).await;
            last_queue_metrics_log = Some(Instant::now());
        }

        let job_opt = match rust_learn::models::upload_job::UploadJob::claim_job(&mut conn).await {
            Ok(job) => job,
            Err(e) => {
                log::error!("event=worker_job_claim_failed error={:?}", e);
                drop(permit);
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
        };

        let Some(job) = job_opt else {
            drop(permit);
            tokio::time::sleep(Duration::from_secs(3)).await;
            continue;
        };

        if !spawn_processing_task(
            pool.clone(),
            s3.clone(),
            notifications.clone(),
            job,
            permit,
            retry_config,
        )
        .await
        {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    runtime::wait_for_in_flight(sem, concurrency).await;
    runtime::join_deposit_indexer(deposit_indexer_handle).await;
    heartbeat_handle.abort();

    Ok(())
}
