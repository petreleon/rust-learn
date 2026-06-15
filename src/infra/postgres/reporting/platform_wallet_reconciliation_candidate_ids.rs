use std::collections::HashSet;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationError;
use crate::db::schema::{reward_candidates, reward_wallet_credit_records};
use crate::infra::postgres::models::wallet::Wallet;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_mappers::map_diesel_error;

pub(super) async fn reward_candidate_ids(
    conn: &mut AsyncPgConnection,
    wallet: &Wallet,
) -> Result<Vec<i64>, PlatformWalletReconciliationError> {
    let credit_candidate_ids: Vec<i64> = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::wallet_id.eq(wallet.id))
        .select(reward_wallet_credit_records::reward_candidate_id)
        .load(conn)
        .await
        .map_err(map_diesel_error)?;

    let mut candidate_ids: HashSet<i64> = credit_candidate_ids.into_iter().collect();
    if let Some(user_id) = wallet.user_id {
        candidate_ids.extend(candidate_ids_for_user(conn, user_id).await?);
    }
    if let Some(organization_id) = wallet.organization_id {
        candidate_ids.extend(candidate_ids_for_organization(conn, organization_id).await?);
    }
    Ok(candidate_ids.into_iter().collect())
}

async fn candidate_ids_for_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Vec<i64>, PlatformWalletReconciliationError> {
    reward_candidates::table
        .filter(reward_candidates::student_user_id.eq(user_id))
        .select(reward_candidates::id)
        .load::<i64>(conn)
        .await
        .map_err(map_diesel_error)
}

async fn candidate_ids_for_organization(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<i64>, PlatformWalletReconciliationError> {
    reward_candidates::table
        .filter(reward_candidates::source_organization_id.eq(organization_id))
        .select(reward_candidates::id)
        .load::<i64>(conn)
        .await
        .map_err(map_diesel_error)
}
