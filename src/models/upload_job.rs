use diesel::prelude::*;
use diesel::sql_types::{BigInt, Varchar, Text, Nullable, Int4, Timestamptz};
use chrono::{DateTime, Utc};
use crate::db::schema::upload_jobs;
use diesel_async::{AsyncPgConnection, RunQueryDsl, AsyncConnection};

#[derive(QueryableByName, Queryable, Identifiable, Selectable, Debug, Clone)]
#[diesel(table_name = upload_jobs)]
pub struct UploadJob {
    #[diesel(sql_type = BigInt)]
    pub id: i64,

    #[diesel(sql_type = Varchar)]
    pub bucket: String,

    #[diesel(sql_type = Text)]
    pub object: String,

    #[diesel(sql_type = Nullable<Int4>)]
    pub user_id: Option<i32>,

    #[diesel(sql_type = Varchar)]
    pub status: String,

    #[diesel(sql_type = Int4)]
    pub attempts: i32,

    #[diesel(sql_type = Nullable<Text>)]
    pub last_error: Option<String>,

    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,

    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = upload_jobs)]
pub struct NewUploadJob<'a> {
    pub bucket: &'a str,
    pub object: &'a str,
    pub user_id: Option<i32>,
}

impl UploadJob {
    pub fn id(&self) -> i64 { self.id }

    pub async fn claim_job(conn: &mut AsyncPgConnection) -> QueryResult<Option<UploadJob>> {
        conn.transaction::<Option<UploadJob>, diesel::result::Error, _>(|tx| Box::pin(async move {
            let candidate: Option<UploadJob> = upload_jobs::table
                .filter(
                    upload_jobs::status.eq("queued")
                        .and(upload_jobs::updated_at.is_null().or(upload_jobs::updated_at.le(Utc::now())))
                )
                .order(upload_jobs::created_at.asc())
                .for_update()
                .skip_locked()
                .first::<UploadJob>(tx)
                .await
                .optional()?;

            if let Some(c) = candidate {
                let claimed = diesel::update(upload_jobs::table.filter(upload_jobs::id.eq(c.id)))
                    .set((upload_jobs::status.eq("processing"), upload_jobs::updated_at.eq(Utc::now())))
                    .get_result::<UploadJob>(tx)
                    .await?;
                Ok(Some(claimed))
            } else {
                Ok(None)
            }
        })).await
    }

    pub async fn mark_done(id: i64, conn: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::update(upload_jobs::table.filter(upload_jobs::id.eq(id)))
            .set((upload_jobs::status.eq("done"), upload_jobs::updated_at.eq(Utc::now())))
            .execute(conn)
            .await
    }

    pub async fn mark_failed(id: i64, attempts: i32, error: String, conn: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::update(upload_jobs::table.filter(upload_jobs::id.eq(id)))
            .set((
                upload_jobs::status.eq("failed"),
                upload_jobs::attempts.eq(attempts),
                upload_jobs::last_error.eq(Some(error)),
                upload_jobs::updated_at.eq(Utc::now()),
            ))
            .execute(conn)
            .await
    }

    pub async fn schedule_retry(id: i64, attempts: i32, error: String, future_time: DateTime<Utc>, conn: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::update(upload_jobs::table.filter(upload_jobs::id.eq(id)))
            .set((
                upload_jobs::status.eq("queued"),
                upload_jobs::attempts.eq(attempts),
                upload_jobs::last_error.eq(Some(error)),
                upload_jobs::updated_at.eq(future_time),
            ))
            .execute(conn)
            .await
    }
}
