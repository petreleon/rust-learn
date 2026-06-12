use crate::config::constants::permissions::Permissions;
use crate::db::schema::users;
use crate::models::kyc_audit_event::{
    KycAuditEvent, NewKycAuditEvent, KYC_AUDIT_EVENT_REVIEW_DECISION, KYC_AUDIT_EVENT_SUBMITTED,
};
use crate::models::kyc_submission::{
    KycSubmission, NewKycSubmission, KYC_STATUS_EXPIRED, KYC_STATUS_PROVIDER_ERROR,
    KYC_STATUS_REJECTED, KYC_STATUS_SUBMITTED, KYC_STATUS_UNDER_REVIEW, KYC_STATUS_VERIFIED,
};
use crate::models::user::User;
use crate::repositories::platform_repository::user_permission_platform_request;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

include!("kyc_service/types.rs");
include!("kyc_service/validation.rs");
include!("kyc_service/audit.rs");
include!("kyc_service/actions.rs");

#[cfg(test)]
mod tests;
