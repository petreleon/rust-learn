use diesel::prelude::*;
use diesel::sql_types::{BigInt, Varchar, Text, Nullable, Int4, Timestamptz};
use chrono::{DateTime, Utc};
use crate::db::schema::upload_jobs;

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
}
