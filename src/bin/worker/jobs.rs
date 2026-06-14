use std::time::Instant;
use tokio::sync::OwnedSemaphorePermit;

use rust_learn::bootstrap::worker_runtime as worker_utils;
use rust_learn::db::DbPool;
use rust_learn::infra::notifications::NotificationsState;
use rust_learn::models::upload_job::UploadJob;
use rust_learn::utils::s3_utils::S3State;

use super::failure::{mark_terminal_failure, schedule_retry};

#[derive(Clone, Copy)]
pub struct RetryConfig {
    pub(super) max_attempts: i64,
    pub(super) base_backoff_seconds: u64,
}

impl RetryConfig {
    pub fn from_env() -> Self {
        RetryConfig {
            max_attempts: worker_utils::positive_i64_from_env_value(
                std::env::var("WORKER_MAX_ATTEMPTS").ok().as_deref(),
                5,
            ),
            base_backoff_seconds: worker_utils::positive_u64_from_env_value(
                std::env::var("WORKER_BASE_BACKOFF_SECONDS").ok().as_deref(),
                60,
            ),
        }
    }
}

pub async fn spawn_processing_task(
    pool: DbPool,
    s3: S3State,
    notifications: NotificationsState,
    job: UploadJob,
    permit: OwnedSemaphorePermit,
    retry_config: RetryConfig,
) -> bool {
    let mut conn_for_task = match pool.get().await {
        Ok(conn) => conn,
        Err(error) => {
            log::error!(
                "event=worker_db_connection_failed phase=task error={:?}",
                error
            );
            drop(permit);
            return false;
        }
    };

    let job_id = job.id;
    let bucket = job.bucket;
    let object = job.object;
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
        retry_config.max_attempts
    );

    tokio::spawn(async move {
        let started_at = Instant::now();
        log::info!(
            "event=worker_job_started job_id={} attempt={} max_attempts={}",
            job_id,
            attempt_number,
            retry_config.max_attempts
        );

        let uid = user_id.unwrap_or(0);
        let result = s3
            .process_uploaded_video(&bucket, &object, uid, notifications.clone())
            .await;
        let duration_ms = started_at.elapsed().as_millis();

        match result {
            Ok(()) => {
                if let Err(error) = UploadJob::mark_done(job_id, &mut conn_for_task).await {
                    log::error!(
                        "event=worker_job_mark_done_failed job_id={} error={:?}",
                        job_id,
                        error
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
            Err(error) => {
                handle_processing_failure(
                    &notifications,
                    &mut conn_for_task,
                    FailureContext {
                        job_id,
                        bucket,
                        object,
                        user_id,
                        duration_ms,
                        current_attempts,
                        retry_config,
                        error: error.to_string(),
                    },
                )
                .await;
            }
        }

        drop(permit);
    });

    true
}

pub(super) struct FailureContext {
    pub(super) job_id: i64,
    pub(super) bucket: String,
    pub(super) object: String,
    pub(super) user_id: Option<i32>,
    pub(super) duration_ms: u128,
    pub(super) current_attempts: i64,
    pub(super) retry_config: RetryConfig,
    pub(super) error: String,
}

async fn handle_processing_failure(
    notifications: &NotificationsState,
    conn: &mut diesel_async::AsyncPgConnection,
    context: FailureContext,
) {
    let new_attempts = context.current_attempts + 1;
    log::warn!(
        "event=worker_job_processing_failed job_id={} bucket={} object={} duration_ms={} attempts={} max_attempts={} error={}",
        context.job_id,
        context.bucket,
        context.object,
        context.duration_ms,
        new_attempts,
        context.retry_config.max_attempts,
        context.error
    );

    if new_attempts >= context.retry_config.max_attempts {
        mark_terminal_failure(notifications, conn, context, new_attempts).await;
    } else {
        schedule_retry(conn, context, new_attempts).await;
    }
}
