use crate::application::rewards::list_platform_candidates::enrichment::{
    candidate_matches_search, enriched_candidates,
};
use crate::application::rewards::list_platform_candidates::store::PlatformRewardCandidateStore;
use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidatePermissions, PlatformRewardCandidatesError,
    PlatformRewardCandidatesOutput, PlatformRewardCandidatesQuery,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub async fn list_platform_reward_candidates(
    store: &mut impl PlatformRewardCandidateStore,
    actor_user_id: i32,
    query: PlatformRewardCandidatesQuery,
) -> Result<PlatformRewardCandidatesOutput, PlatformRewardCandidatesError> {
    ensure_can_view_candidates(store, actor_user_id).await?;
    let status = normalized_status(query.status)?;
    let search = normalized_search(query.search);
    let limit = query.limit.unwrap_or(25).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).max(0);

    let can_approve_amount = store.can_approve_reward_amount(actor_user_id).await?;
    let records = store.list_candidate_records(status.clone()).await?;
    store.count_candidate_records(status.clone()).await?;

    let mut candidates = enriched_candidates(store, records).await?;
    if let Some(ref search) = search {
        candidates.retain(|candidate| candidate_matches_search(candidate, search));
    }
    let total = candidates.len() as i64;
    let candidates = candidates
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    Ok(PlatformRewardCandidatesOutput {
        candidates,
        total,
        limit,
        offset,
        status,
        search,
        operator_permissions: PlatformRewardCandidatePermissions {
            can_view_candidates: true,
            can_approve_amount,
        },
    })
}

async fn ensure_can_view_candidates(
    store: &mut impl PlatformRewardCandidateStore,
    actor_user_id: i32,
) -> Result<(), PlatformRewardCandidatesError> {
    if store.can_view_reward_audit(actor_user_id).await? {
        Ok(())
    } else {
        Err(PlatformRewardCandidatesError::PermissionDenied(
            "VIEW_REWARD_AUDIT".to_string(),
        ))
    }
}

fn normalized_status(
    status: Option<String>,
) -> Result<Option<String>, PlatformRewardCandidatesError> {
    status
        .map(|status| {
            RewardCandidateStatus::normalize(&status).map_err(|_| {
                PlatformRewardCandidatesError::InvalidStatus(
                    "unsupported reward candidate status".to_string(),
                )
            })
        })
        .transpose()
}

fn normalized_search(search: Option<String>) -> Option<String> {
    search
        .map(|search| search.trim().to_lowercase())
        .filter(|search| !search.is_empty())
}

#[cfg(test)]
mod tests;
