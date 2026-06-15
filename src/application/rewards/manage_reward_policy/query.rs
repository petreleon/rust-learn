use crate::domain::rewards::policy::{RewardPolicyEventType, RewardPolicyScope};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ListRewardPoliciesQuery {
    pub scope_type: Option<RewardPolicyScope>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: Option<RewardPolicyEventType>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RewardPolicyListFilter {
    pub scope_type: Option<RewardPolicyScope>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: Option<RewardPolicyEventType>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
