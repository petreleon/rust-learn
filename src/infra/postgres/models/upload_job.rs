use crate::infra::postgres::schema::upload_jobs;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Int4, Nullable, Text, Timestamptz, Varchar};

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

impl UploadJob {
    pub fn id(&self) -> i64 {
        self.id
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = upload_jobs)]
pub struct NewUploadJob<'a> {
    pub bucket: &'a str,
    pub object: &'a str,
    pub user_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadJobQueueMetrics {
    pub queued_ready: i64,
    pub queued_delayed: i64,
    pub processing: i64,
    pub failed: i64,
}

impl UploadJobQueueMetrics {
    pub fn queue_depth(&self) -> i64 {
        self.queued_ready + self.queued_delayed
    }
}
