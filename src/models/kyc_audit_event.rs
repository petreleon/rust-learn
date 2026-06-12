use crate::db::schema::kyc_audit_events;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;
use serde_json::Value;

pub const KYC_AUDIT_EVENT_REVIEW_DECISION: &str = "review_decision";
pub const KYC_AUDIT_EVENT_SUBMITTED: &str = "submitted";

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = kyc_audit_events)]
pub struct KycAuditEvent {
    pub id: i64,
    pub submission_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = kyc_audit_events)]
pub struct NewKycAuditEvent {
    pub submission_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
}

impl KycAuditEvent {
    pub async fn create(
        conn: &mut AsyncPgConnection,
        event: NewKycAuditEvent,
    ) -> QueryResult<KycAuditEvent> {
        diesel::insert_into(kyc_audit_events::table)
            .values(event)
            .get_result(conn)
            .await
    }

    pub async fn list_for_submission(
        conn: &mut AsyncPgConnection,
        submission_id: i64,
    ) -> QueryResult<Vec<KycAuditEvent>> {
        kyc_audit_events::table
            .filter(kyc_audit_events::submission_id.eq(submission_id))
            .order(kyc_audit_events::created_at.asc())
            .then_order_by(kyc_audit_events::id.asc())
            .load(conn)
            .await
    }
}
