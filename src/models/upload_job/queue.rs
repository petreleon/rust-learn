use super::{UploadJob, UploadJobQueueMetrics};
use crate::db::schema::upload_jobs;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

impl UploadJob {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub async fn claim_job(conn: &mut AsyncPgConnection) -> QueryResult<Option<UploadJob>> {
        conn.transaction::<Option<UploadJob>, diesel::result::Error, _>(|tx| {
            Box::pin(async move {
                let candidate = upload_jobs::table
                    .filter(
                        upload_jobs::status.eq("queued").and(
                            upload_jobs::updated_at
                                .is_null()
                                .or(upload_jobs::updated_at.le(Utc::now())),
                        ),
                    )
                    .order(upload_jobs::created_at.asc())
                    .for_update()
                    .skip_locked()
                    .first::<UploadJob>(tx)
                    .await
                    .optional()?;

                match candidate {
                    Some(candidate) => claim_candidate(candidate, tx).await.map(Some),
                    None => Ok(None),
                }
            })
        })
        .await
    }

    pub async fn queue_metrics(conn: &mut AsyncPgConnection) -> QueryResult<UploadJobQueueMetrics> {
        let now = Utc::now();
        let ready_filter = upload_jobs::status.eq("queued").and(
            upload_jobs::updated_at
                .is_null()
                .or(upload_jobs::updated_at.le(now)),
        );

        let queued_ready = upload_jobs::table
            .filter(ready_filter)
            .count()
            .get_result(conn)
            .await?;
        let queued_delayed = upload_jobs::table
            .filter(
                upload_jobs::status
                    .eq("queued")
                    .and(upload_jobs::updated_at.gt(now)),
            )
            .count()
            .get_result(conn)
            .await?;
        let processing = count_status(conn, "processing").await?;
        let failed = count_status(conn, "failed").await?;

        Ok(UploadJobQueueMetrics {
            queued_ready,
            queued_delayed,
            processing,
            failed,
        })
    }

    pub async fn mark_done(id: i64, conn: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::update(upload_jobs::table.filter(upload_jobs::id.eq(id)))
            .set((
                upload_jobs::status.eq("done"),
                upload_jobs::updated_at.eq(Utc::now()),
            ))
            .execute(conn)
            .await
    }

    pub async fn mark_failed(
        id: i64,
        attempts: i32,
        error: String,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<usize> {
        update_failed_or_retry(id, attempts, error, Utc::now(), "failed", conn).await
    }

    pub async fn schedule_retry(
        id: i64,
        attempts: i32,
        error: String,
        future_time: DateTime<Utc>,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<usize> {
        update_failed_or_retry(id, attempts, error, future_time, "queued", conn).await
    }
}

async fn claim_candidate(
    candidate: UploadJob,
    conn: &mut AsyncPgConnection,
) -> QueryResult<UploadJob> {
    diesel::update(upload_jobs::table.filter(upload_jobs::id.eq(candidate.id)))
        .set((
            upload_jobs::status.eq("processing"),
            upload_jobs::updated_at.eq(Utc::now()),
        ))
        .get_result::<UploadJob>(conn)
        .await
}

async fn count_status(conn: &mut AsyncPgConnection, status: &str) -> QueryResult<i64> {
    upload_jobs::table
        .filter(upload_jobs::status.eq(status))
        .count()
        .get_result(conn)
        .await
}

async fn update_failed_or_retry(
    id: i64,
    attempts: i32,
    error: String,
    updated_at: DateTime<Utc>,
    status: &str,
    conn: &mut AsyncPgConnection,
) -> QueryResult<usize> {
    diesel::update(upload_jobs::table.filter(upload_jobs::id.eq(id)))
        .set((
            upload_jobs::status.eq(status),
            upload_jobs::attempts.eq(attempts),
            upload_jobs::last_error.eq(Some(error)),
            upload_jobs::updated_at.eq(updated_at),
        ))
        .execute(conn)
        .await
}
