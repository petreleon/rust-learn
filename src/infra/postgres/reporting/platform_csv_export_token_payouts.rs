use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_csv_exports::{
    platform_token_payout_export_row, PlatformCsvExportError, PlatformTokenPayoutExportFact,
    PlatformTokenPayoutExportRowOutput,
};
use crate::domain::rewards::token::RewardTokenEventType;
use crate::infra::postgres::models::reward_candidate::RewardCandidate;
use crate::infra::postgres::models::reward_payout_record::RewardPayoutRecord;
use crate::infra::postgres::reporting::platform_csv_export_mappers::map_diesel_error;
use crate::infra::postgres::schema::{
    external_transactions, reward_candidates, reward_payout_records,
};

type ExternalTransactionRow = (
    BigDecimal,
    String,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<String>,
);

pub(super) async fn load_token_payout_export_rows(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<PlatformTokenPayoutExportRowOutput>, PlatformCsvExportError> {
    let payout_records = reward_payout_records::table
        .order(reward_payout_records::created_at.desc())
        .limit(1000)
        .load::<RewardPayoutRecord>(conn)
        .await
        .map_err(map_diesel_error)?;
    let mut rows = Vec::new();
    for record in payout_records {
        rows.push(token_payout_row(conn, record).await?);
    }
    Ok(rows)
}

async fn token_payout_row(
    conn: &mut AsyncPgConnection,
    record: RewardPayoutRecord,
) -> Result<PlatformTokenPayoutExportRowOutput, PlatformCsvExportError> {
    let candidate = reward_candidates::table
        .find(record.reward_candidate_id)
        .first::<RewardCandidate>(conn)
        .await
        .map_err(map_diesel_error)?;
    let external = external_transactions::table
        .find(record.external_transaction_id)
        .select((
            external_transactions::amount,
            external_transactions::blockchain_address,
            external_transactions::chain_id,
            external_transactions::contract_address,
            external_transactions::transaction_hash,
            external_transactions::log_index,
            external_transactions::event_type,
            external_transactions::from_address,
            external_transactions::to_address,
        ))
        .first::<ExternalTransactionRow>(conn)
        .await
        .map_err(map_diesel_error)?;
    let fact = token_payout_fact(record, candidate, external)?;
    Ok(platform_token_payout_export_row(fact))
}

fn token_payout_fact(
    record: RewardPayoutRecord,
    candidate: RewardCandidate,
    external: ExternalTransactionRow,
) -> Result<PlatformTokenPayoutExportFact, PlatformCsvExportError> {
    let event_type = RewardTokenEventType::parse_optional(external.6)
        .map_err(|error| PlatformCsvExportError::Database(error.to_string()))?;
    Ok(PlatformTokenPayoutExportFact {
        reward_payout_record_id: record.id,
        reward_candidate_id: record.reward_candidate_id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        payout_transaction_id: record.transaction_id,
        external_transaction_id: record.external_transaction_id,
        amount: external.0,
        blockchain_address: external.1,
        chain_id: external.2,
        contract_address: external.3,
        transaction_hash: external.4,
        log_index: external.5,
        event_type,
        from_address: external.7,
        to_address: external.8,
        created_at: record.created_at,
    })
}
