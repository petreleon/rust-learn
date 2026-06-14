use chrono::Utc;
use diesel_async::AsyncPgConnection;

use super::jobs::FailureContext;
use rust_learn::bootstrap::worker_runtime as worker_utils;
use rust_learn::models::upload_job::UploadJob;
use rust_learn::utils::notifications::NotificationsState;

pub async fn mark_terminal_failure(
    notifications: &NotificationsState,
    conn: &mut AsyncPgConnection,
    context: FailureContext,
    new_attempts: i64,
) {
    if let Some(uid) = context.user_id {
        if let Err(error) = notifications
            .send_worker_failure_notification(
                uid,
                context.job_id,
                &context.object,
                new_attempts as i32,
                &context.error,
            )
            .await
        {
            log::warn!(
                "event=notification_send_failed kind=worker_failure job_id={} user_id={} error={:?}",
                context.job_id,
                uid,
                error
            );
        }
    }

    if let Err(error) = UploadJob::mark_failed(
        context.job_id,
        new_attempts as i32,
        context.error.clone(),
        conn,
    )
    .await
    {
        log::error!(
            "event=worker_job_mark_failed_failed job_id={} attempts={} error={:?}",
            context.job_id,
            new_attempts,
            error
        );
    }
    log::warn!(
        "event=worker_job_terminal_failure job_id={} duration_ms={} attempts={} max_attempts={} failed_jobs_delta=1",
        context.job_id,
        context.duration_ms,
        new_attempts,
        context.retry_config.max_attempts
    );
}

pub async fn schedule_retry(
    conn: &mut AsyncPgConnection,
    context: FailureContext,
    new_attempts: i64,
) {
    let future_time = worker_utils::retry_available_at(
        Utc::now(),
        context.retry_config.base_backoff_seconds,
        context.current_attempts,
    );

    if let Err(error) = UploadJob::schedule_retry(
        context.job_id,
        new_attempts as i32,
        context.error.clone(),
        future_time,
        conn,
    )
    .await
    {
        log::error!(
            "event=worker_job_schedule_retry_failed job_id={} attempts={} error={:?}",
            context.job_id,
            new_attempts,
            error
        );
    }
    log::info!(
        "event=worker_job_retry_scheduled job_id={} duration_ms={} attempts={} max_attempts={} retry_available_at={}",
        context.job_id,
        context.duration_ms,
        new_attempts,
        context.retry_config.max_attempts,
        future_time
    );
}
