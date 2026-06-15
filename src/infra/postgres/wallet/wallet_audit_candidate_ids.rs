use std::collections::HashSet;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::audit_wallet::{WalletAuditError, WalletAuditTarget};
use crate::infra::postgres::schema::{reward_candidates, reward_wallet_credit_records};
use crate::infra::postgres::wallet::wallet_audit_mappers::map_wallet_audit_error;

pub(super) async fn load_wallet_reward_candidate_ids(
    conn: &mut AsyncPgConnection,
    target: &WalletAuditTarget,
) -> Result<Vec<i64>, WalletAuditError> {
    let mut candidate_ids = HashSet::new();

    let credit_candidate_ids = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::wallet_id.eq(target.id))
        .select(reward_wallet_credit_records::reward_candidate_id)
        .load::<i64>(conn)
        .await
        .map_err(map_wallet_audit_error)?;
    candidate_ids.extend(credit_candidate_ids);

    if let Some(user_id) = target.user_id {
        let user_candidate_ids = reward_candidates::table
            .filter(reward_candidates::student_user_id.eq(user_id))
            .select(reward_candidates::id)
            .load::<i64>(conn)
            .await
            .map_err(map_wallet_audit_error)?;
        candidate_ids.extend(user_candidate_ids);
    }

    if let Some(organization_id) = target.organization_id {
        let organization_candidate_ids = reward_candidates::table
            .filter(reward_candidates::source_organization_id.eq(organization_id))
            .select(reward_candidates::id)
            .load::<i64>(conn)
            .await
            .map_err(map_wallet_audit_error)?;
        candidate_ids.extend(organization_candidate_ids);
    }

    let mut candidate_ids = candidate_ids.into_iter().collect::<Vec<_>>();
    candidate_ids.sort_unstable();
    Ok(candidate_ids)
}
