use std::time::Duration;
use tokio::sync::Semaphore;

use rust_learn::models::upload_job::UploadJob;

pub async fn log_queue_metrics(
    conn: &mut diesel_async::AsyncPgConnection,
    sem: &Semaphore,
    concurrency: usize,
    interval: Duration,
) {
    match UploadJob::queue_metrics(conn).await {
        Ok(metrics) => {
            let in_flight = concurrency
                .saturating_sub(sem.available_permits())
                .saturating_sub(1);
            log::info!(
                "event=worker_queue_metrics queue_depth={} queued_ready={} queued_delayed={} processing={} failed={} in_flight={} concurrency={} interval_seconds={}",
                metrics.queue_depth(),
                metrics.queued_ready,
                metrics.queued_delayed,
                metrics.processing,
                metrics.failed,
                in_flight,
                concurrency,
                interval.as_secs()
            );
        }
        Err(error) => {
            log::warn!("event=worker_queue_metrics_failed error={:?}", error);
        }
    }
}
