use crate::db::schema::kyc_submissions;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

pub const KYC_STATUS_EXPIRED: &str = "expired";
pub const KYC_STATUS_PROVIDER_ERROR: &str = "provider_error";
pub const KYC_STATUS_REJECTED: &str = "rejected";
pub const KYC_STATUS_SUBMITTED: &str = "submitted";
pub const KYC_STATUS_UNDER_REVIEW: &str = "under_review";
pub const KYC_STATUS_VERIFIED: &str = "verified";

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = kyc_submissions)]
pub struct KycSubmission {
    pub id: i64,
    pub user_id: i32,
    pub status: String,
    pub legal_name: String,
    pub country_code: String,
    pub document_type: String,
    pub document_last4: Option<String>,
    pub evidence_reference: Option<String>,
    pub provider_reference: Option<String>,
    pub reviewer_user_id: Option<i32>,
    pub rejection_reason: Option<String>,
    pub submitted_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = kyc_submissions)]
pub struct NewKycSubmission {
    pub user_id: i32,
    pub status: String,
    pub legal_name: String,
    pub country_code: String,
    pub document_type: String,
    pub document_last4: Option<String>,
    pub evidence_reference: Option<String>,
    pub provider_reference: Option<String>,
}

impl KycSubmission {
    pub async fn create(
        conn: &mut AsyncPgConnection,
        new_submission: NewKycSubmission,
    ) -> QueryResult<KycSubmission> {
        diesel::insert_into(kyc_submissions::table)
            .values(&new_submission)
            .get_result(conn)
            .await
    }

    pub async fn find_by_id(conn: &mut AsyncPgConnection, id: i64) -> QueryResult<KycSubmission> {
        kyc_submissions::table.find(id).first(conn).await
    }

    pub async fn latest_for_user(
        conn: &mut AsyncPgConnection,
        user_id: i32,
    ) -> QueryResult<Option<KycSubmission>> {
        kyc_submissions::table
            .filter(kyc_submissions::user_id.eq(user_id))
            .order(kyc_submissions::updated_at.desc())
            .then_order_by(kyc_submissions::id.desc())
            .first(conn)
            .await
            .optional()
    }

    pub async fn list_review_queue(
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<KycSubmission>> {
        kyc_submissions::table
            .filter(kyc_submissions::status.eq_any([KYC_STATUS_SUBMITTED, KYC_STATUS_UNDER_REVIEW]))
            .order(kyc_submissions::submitted_at.asc())
            .then_order_by(kyc_submissions::id.asc())
            .limit(50)
            .load(conn)
            .await
    }

    pub async fn decide(
        conn: &mut AsyncPgConnection,
        id: i64,
        reviewer_user_id: i32,
        status: &str,
        rejection_reason: Option<String>,
    ) -> QueryResult<KycSubmission> {
        let now = Utc::now();
        diesel::update(kyc_submissions::table.find(id))
            .set((
                kyc_submissions::status.eq(status),
                kyc_submissions::reviewer_user_id.eq(Some(reviewer_user_id)),
                kyc_submissions::rejection_reason.eq(rejection_reason),
                kyc_submissions::reviewed_at.eq(Some(now)),
                kyc_submissions::updated_at.eq(now),
            ))
            .get_result(conn)
            .await
    }
}
