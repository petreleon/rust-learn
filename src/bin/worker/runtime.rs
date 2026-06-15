use anyhow::{anyhow, Result};
use dotenvy::dotenv;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;

use rust_learn::bootstrap::worker_runtime as worker_utils;
use rust_learn::infra::object_storage::S3State;
use rust_learn::infra::postgres::DbPool;

pub fn init_environment() {
    dotenv().ok();
    rust_learn::bootstrap::logging::init_logging("worker");
}

pub fn init_pool() -> Result<DbPool> {
    rust_learn::infra::postgres::try_establish_connection().map_err(|error| {
        log::error!("event=worker_db_pool_init_failed error={}", error);
        anyhow!(error)
    })
}

pub async fn init_s3() -> Result<S3State> {
    S3State::new_from_env().await.map_err(|error| {
        log::error!("event=worker_s3_init_failed error={:?}", error);
        error
    })
}

pub fn spawn_shutdown_listener() -> Arc<AtomicBool> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_handle = shutdown.clone();

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

    shutdown
}

pub fn worker_concurrency() -> usize {
    worker_utils::positive_usize_from_env_value(
        std::env::var("WORKER_CONCURRENCY").ok().as_deref(),
        1,
    )
}

pub fn queue_metrics_interval(default: Duration) -> Duration {
    Duration::from_secs(worker_utils::positive_u64_from_env_value(
        std::env::var("WORKER_QUEUE_METRICS_INTERVAL_SECONDS")
            .ok()
            .as_deref(),
        default.as_secs(),
    ))
}

pub fn spawn_heartbeat(interval: Duration) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            if let Err(error) =
                worker_utils::write_heartbeat(worker_utils::DEFAULT_WORKER_HEARTBEAT_PATH).await
            {
                log::warn!("event=worker_heartbeat_write_failed error={:?}", error);
            }
            tokio::time::sleep(interval).await;
        }
    })
}

pub async fn wait_for_in_flight(sem: Arc<Semaphore>, concurrency: usize) {
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
}

pub async fn join_deposit_indexer(handle: Option<JoinHandle<()>>) {
    if let Some(handle) = handle {
        if let Err(error) = handle.await {
            log::warn!("event=wallet_deposit_indexer_join_failed error={:?}", error);
        }
    }
}
