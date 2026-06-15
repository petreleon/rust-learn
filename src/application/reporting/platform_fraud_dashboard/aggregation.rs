use chrono::{DateTime, Utc};

use crate::application::reporting::platform_fraud_dashboard::{
    FraudBlockDashboardRowOutput, FraudBlockScopeSummaryOutput, PlatformFraudDashboardOutput,
};
use crate::domain::rewards::fraud_block::RewardFraudBlockScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FraudBlockDashboardFact {
    pub id: i64,
    pub scope_type: RewardFraudBlockScope,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub created_by_user_id: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) fn platform_fraud_dashboard_from_facts(
    facts: Vec<FraudBlockDashboardFact>,
) -> PlatformFraudDashboardOutput {
    let active_by_scope = scope_summary(&facts);
    PlatformFraudDashboardOutput {
        active_total: facts.len() as i64,
        active_by_scope,
        active_blocks: facts.into_iter().map(fraud_block_row).collect(),
    }
}

fn scope_summary(facts: &[FraudBlockDashboardFact]) -> FraudBlockScopeSummaryOutput {
    let mut summary = FraudBlockScopeSummaryOutput::default();
    for fact in facts {
        match fact.scope_type {
            RewardFraudBlockScope::Teacher => summary.teacher += 1,
            RewardFraudBlockScope::Organization => summary.organization += 1,
            RewardFraudBlockScope::Course => summary.course += 1,
            RewardFraudBlockScope::RewardPolicy => summary.reward_policy += 1,
        }
    }
    summary
}

fn fraud_block_row(fact: FraudBlockDashboardFact) -> FraudBlockDashboardRowOutput {
    FraudBlockDashboardRowOutput {
        id: fact.id,
        scope_type: fact.scope_type,
        teacher_user_id: fact.teacher_user_id,
        organization_id: fact.organization_id,
        course_id: fact.course_id,
        reward_policy_id: fact.reward_policy_id,
        reason: fact.reason,
        evidence_reference: fact.evidence_reference,
        created_by_user_id: fact.created_by_user_id,
        expires_at: fact.expires_at,
        created_at: fact.created_at,
        updated_at: fact.updated_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_dashboard_summary_and_rows_from_facts() {
        let now = Utc::now();

        let dashboard = platform_fraud_dashboard_from_facts(vec![
            fraud_block_fact(1, RewardFraudBlockScope::Teacher, now),
            fraud_block_fact(2, RewardFraudBlockScope::Organization, now),
            fraud_block_fact(3, RewardFraudBlockScope::Course, now),
            fraud_block_fact(4, RewardFraudBlockScope::RewardPolicy, now),
        ]);

        assert_eq!(dashboard.active_total, 4);
        assert_eq!(dashboard.active_by_scope.teacher, 1);
        assert_eq!(dashboard.active_by_scope.organization, 1);
        assert_eq!(dashboard.active_by_scope.course, 1);
        assert_eq!(dashboard.active_by_scope.reward_policy, 1);
        assert_eq!(dashboard.active_blocks.len(), 4);
        assert_eq!(dashboard.active_blocks[0].reason, "block 1");
        assert_eq!(
            dashboard.active_blocks[3].scope_type,
            RewardFraudBlockScope::RewardPolicy
        );
    }

    #[test]
    fn empty_facts_produce_zero_summary_and_empty_rows() {
        let dashboard = platform_fraud_dashboard_from_facts(vec![]);

        assert_eq!(dashboard.active_total, 0);
        assert_eq!(
            dashboard.active_by_scope,
            FraudBlockScopeSummaryOutput::default()
        );
        assert!(dashboard.active_blocks.is_empty());
    }

    fn fraud_block_fact(
        id: i64,
        scope_type: RewardFraudBlockScope,
        now: DateTime<Utc>,
    ) -> FraudBlockDashboardFact {
        FraudBlockDashboardFact {
            id,
            scope_type,
            teacher_user_id: Some(10),
            organization_id: Some(20),
            course_id: Some(30),
            reward_policy_id: Some(40),
            reason: format!("block {id}"),
            evidence_reference: Some(format!("evidence-{id}")),
            created_by_user_id: 1,
            expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}
