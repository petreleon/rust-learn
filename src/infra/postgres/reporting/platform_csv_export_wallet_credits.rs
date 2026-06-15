use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_csv_exports::{
    platform_wallet_credit_export_row, PlatformCsvExportError, PlatformWalletCreditExportFact,
    PlatformWalletCreditExportRowOutput,
};
use crate::infra::postgres::models::reward_candidate::RewardCandidate;
use crate::infra::postgres::models::reward_wallet_credit_record::RewardWalletCreditRecord;
use crate::infra::postgres::reporting::platform_csv_export_mappers::map_diesel_error;
use crate::infra::postgres::schema::{
    internal_transactions, reward_candidates, reward_wallet_credit_records,
};

pub(super) async fn load_wallet_credit_export_rows(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<PlatformWalletCreditExportRowOutput>, PlatformCsvExportError> {
    let credit_records = reward_wallet_credit_records::table
        .order(reward_wallet_credit_records::created_at.desc())
        .limit(1000)
        .load::<RewardWalletCreditRecord>(conn)
        .await
        .map_err(map_diesel_error)?;
    let mut rows = Vec::new();
    for record in credit_records {
        rows.push(wallet_credit_row(conn, record).await?);
    }
    Ok(rows)
}

async fn wallet_credit_row(
    conn: &mut AsyncPgConnection,
    record: RewardWalletCreditRecord,
) -> Result<PlatformWalletCreditExportRowOutput, PlatformCsvExportError> {
    let candidate = reward_candidates::table
        .find(record.reward_candidate_id)
        .first::<RewardCandidate>(conn)
        .await
        .map_err(map_diesel_error)?;
    let amount = internal_transactions::table
        .find(record.internal_transaction_id)
        .select(internal_transactions::amount)
        .first::<BigDecimal>(conn)
        .await
        .map_err(map_diesel_error)?;
    Ok(platform_wallet_credit_export_row(wallet_credit_fact(
        record, candidate, amount,
    )))
}

fn wallet_credit_fact(
    record: RewardWalletCreditRecord,
    candidate: RewardCandidate,
    amount: BigDecimal,
) -> PlatformWalletCreditExportFact {
    PlatformWalletCreditExportFact {
        reward_wallet_credit_record_id: record.id,
        reward_candidate_id: record.reward_candidate_id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        wallet_id: record.wallet_id,
        transaction_id: record.transaction_id,
        internal_transaction_id: record.internal_transaction_id,
        amount,
        notification_id: record.notification_id,
        notified_at: record.notified_at,
        created_at: record.created_at,
    }
}
