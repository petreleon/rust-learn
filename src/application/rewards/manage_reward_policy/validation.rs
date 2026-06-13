use bigdecimal::BigDecimal;

use crate::application::rewards::manage_reward_policy::{
    CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyDraft, RewardPolicyError,
    RewardPolicyListFilter,
};
use crate::application::rewards::ports::RewardPolicyStore;
use crate::domain::rewards::policy::{
    normalize_event_type, normalize_payment_strategy, normalize_scope_type,
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};

pub(super) async fn validated_draft(
    store: &mut impl RewardPolicyStore,
    actor_user_id: i32,
    command: CreateRewardPolicyCommand,
) -> Result<RewardPolicyDraft, RewardPolicyError> {
    let scope_type = normalize_scope_type(&command.scope_type).ok_or_else(|| {
        RewardPolicyError::InvalidInput("unsupported reward policy scope type".to_string())
    })?;
    validate_scope_references(
        store,
        &scope_type,
        command.organization_id,
        command.course_id,
    )
    .await?;
    let event_type = normalize_event_type(&command.event_type).ok_or_else(|| {
        RewardPolicyError::InvalidInput("unsupported reward policy event type".to_string())
    })?;
    let payment_strategy =
        normalize_payment_strategy(&command.payment_strategy).ok_or_else(|| {
            RewardPolicyError::InvalidInput(
                "unsupported reward policy payment strategy".to_string(),
            )
        })?;
    let multiplier = command.multiplier.unwrap_or_else(|| BigDecimal::from(1));
    let cooldown_seconds = command.cooldown_seconds.unwrap_or(0);
    validate_amounts(
        &command.token_amount,
        &multiplier,
        command.max_payout.as_ref(),
        cooldown_seconds,
    )?;

    Ok(RewardPolicyDraft {
        actor_user_id,
        scope_type,
        organization_id: command.organization_id,
        course_id: command.course_id,
        event_type,
        token_amount: command.token_amount,
        multiplier,
        max_payout: command.max_payout,
        cooldown_seconds,
        payment_strategy,
        active: command.active.unwrap_or(true),
    })
}

pub(super) fn validated_filter(
    query: ListRewardPoliciesQuery,
) -> Result<RewardPolicyListFilter, RewardPolicyError> {
    Ok(RewardPolicyListFilter {
        scope_type: query.scope_type.map(normalized_scope).transpose()?,
        organization_id: query.organization_id,
        course_id: query.course_id,
        event_type: query.event_type.map(normalized_event).transpose()?,
        active: query.active,
        limit: query.limit,
        offset: query.offset,
    })
}

async fn validate_scope_references(
    store: &mut impl RewardPolicyStore,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(), RewardPolicyError> {
    match scope_type {
        REWARD_POLICY_SCOPE_PLATFORM => reject_platform_refs(organization_id, course_id),
        REWARD_POLICY_SCOPE_ORGANIZATION => {
            validate_organization_scope(store, organization_id, course_id).await
        }
        REWARD_POLICY_SCOPE_COURSE => {
            validate_course_scope(store, organization_id, course_id).await
        }
        _ => unreachable!("scope type was normalized before validation"),
    }
}

fn reject_platform_refs(
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(), RewardPolicyError> {
    if organization_id.is_some() || course_id.is_some() {
        return Err(RewardPolicyError::InvalidInput(
            "platform reward policy cannot include organization or course scope".to_string(),
        ));
    }
    Ok(())
}

async fn validate_organization_scope(
    store: &mut impl RewardPolicyStore,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(), RewardPolicyError> {
    let organization_id = organization_id.ok_or_else(|| {
        RewardPolicyError::InvalidInput(
            "organization reward policy requires organization_id".to_string(),
        )
    })?;
    if course_id.is_some() {
        return Err(RewardPolicyError::InvalidInput(
            "organization reward policy cannot include course_id".to_string(),
        ));
    }
    store.organization_exists(organization_id).await
}

async fn validate_course_scope(
    store: &mut impl RewardPolicyStore,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(), RewardPolicyError> {
    let course_id = course_id.ok_or_else(|| {
        RewardPolicyError::InvalidInput("course reward policy requires course_id".to_string())
    })?;
    store.course_exists(course_id).await?;
    if let Some(organization_id) = organization_id {
        store.organization_exists(organization_id).await?;
    }
    Ok(())
}

fn validate_amounts(
    token_amount: &BigDecimal,
    multiplier: &BigDecimal,
    max_payout: Option<&BigDecimal>,
    cooldown_seconds: i64,
) -> Result<(), RewardPolicyError> {
    if token_amount < &BigDecimal::from(0) {
        return Err(RewardPolicyError::InvalidInput(
            "token amount cannot be negative".to_string(),
        ));
    }
    if multiplier <= &BigDecimal::from(0) {
        return Err(RewardPolicyError::InvalidInput(
            "multiplier must be greater than zero".to_string(),
        ));
    }
    if max_payout.is_some_and(|max_payout| max_payout < &BigDecimal::from(0)) {
        return Err(RewardPolicyError::InvalidInput(
            "max payout cannot be negative".to_string(),
        ));
    }
    if cooldown_seconds < 0 {
        return Err(RewardPolicyError::InvalidInput(
            "cooldown seconds cannot be negative".to_string(),
        ));
    }
    Ok(())
}

fn normalized_scope(scope: String) -> Result<String, RewardPolicyError> {
    normalize_scope_type(&scope).ok_or_else(|| {
        RewardPolicyError::InvalidInput("unsupported reward policy scope type".to_string())
    })
}

fn normalized_event(event: String) -> Result<String, RewardPolicyError> {
    normalize_event_type(&event).ok_or_else(|| {
        RewardPolicyError::InvalidInput("unsupported reward policy event type".to_string())
    })
}

#[cfg(test)]
mod tests;
