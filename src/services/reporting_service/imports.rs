use crate::db::schema::{
    delegated_permissions, external_transactions, internal_transactions, reward_candidates,
    reward_payout_records, reward_wallet_credit_records, teacher_applications,
};
use crate::models::delegated_permission::DelegatedPermission;
use crate::models::reward_candidate::RewardCandidate;
use crate::models::reward_payout_record::RewardPayoutRecord;
use crate::models::reward_wallet_credit_record::RewardWalletCreditRecord;
use crate::models::teacher_application::TeacherApplication;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

fn csv_value(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn csv_optional(value: Option<impl ToString>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}
