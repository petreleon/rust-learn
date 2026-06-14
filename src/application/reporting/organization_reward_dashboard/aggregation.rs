use bigdecimal::BigDecimal;

use crate::application::reporting::organization_reward_dashboard::{
    OrganizationCourseRewardDashboardRowOutput, OrganizationRewardDashboardOutput,
    OrganizationWalletBalanceRowOutput, TeacherApplicationDashboardSummaryOutput,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OrganizationRewardDashboardFacts {
    pub organization_id: i32,
    pub organization_name: String,
    pub sponsored_teacher_applications: TeacherApplicationDashboardSummaryOutput,
    pub courses: Vec<OrganizationCourseRewardDashboardFact>,
    pub wallets: Vec<OrganizationWalletBalanceFact>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OrganizationCourseRewardDashboardFact {
    pub row: OrganizationCourseRewardDashboardRowOutput,
    pub approved_amount_total: BigDecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OrganizationWalletBalanceFact {
    pub row: OrganizationWalletBalanceRowOutput,
    pub balance: BigDecimal,
}

pub(crate) fn organization_reward_dashboard_from_facts(
    facts: OrganizationRewardDashboardFacts,
) -> OrganizationRewardDashboardOutput {
    let course_reward_count = facts
        .courses
        .iter()
        .map(|data| data.row.reward_candidate_count)
        .sum::<i64>();
    let approved_reward_count = facts
        .courses
        .iter()
        .map(|data| data.row.approved_reward_count)
        .sum::<i64>();
    let approved_amount_total = facts.courses.iter().fold(BigDecimal::from(0), |sum, data| {
        sum + data.approved_amount_total.clone()
    });
    let wallet_balance_total = facts
        .wallets
        .iter()
        .fold(BigDecimal::from(0), |sum, data| sum + data.balance.clone());

    OrganizationRewardDashboardOutput {
        organization_id: facts.organization_id,
        organization_name: facts.organization_name,
        sponsored_teacher_applications: facts.sponsored_teacher_applications,
        course_reward_count,
        approved_reward_count,
        approved_amount_total: approved_amount_total.to_string(),
        courses: facts.courses.into_iter().map(|data| data.row).collect(),
        wallets: facts.wallets.into_iter().map(|data| data.row).collect(),
        wallet_balance_total: wallet_balance_total.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use super::*;

    #[test]
    fn builds_dashboard_totals_from_fact_rows() {
        let dashboard =
            organization_reward_dashboard_from_facts(OrganizationRewardDashboardFacts {
                organization_id: 42,
                organization_name: "Org".to_string(),
                sponsored_teacher_applications: TeacherApplicationDashboardSummaryOutput {
                    total: 2,
                    submitted: 1,
                    approved: 1,
                    ..TeacherApplicationDashboardSummaryOutput::default()
                },
                courses: vec![
                    course_fact(1, 3, 2, BigDecimal::from(12)),
                    course_fact(2, 4, 1, BigDecimal::from(8)),
                ],
                wallets: vec![
                    wallet_fact(10, BigDecimal::from(5)),
                    wallet_fact(11, BigDecimal::from(7)),
                ],
            });

        assert_eq!(dashboard.organization_id, 42);
        assert_eq!(dashboard.organization_name, "Org");
        assert_eq!(dashboard.sponsored_teacher_applications.total, 2);
        assert_eq!(dashboard.course_reward_count, 7);
        assert_eq!(dashboard.approved_reward_count, 3);
        assert_eq!(dashboard.approved_amount_total, "20");
        assert_eq!(dashboard.courses.len(), 2);
        assert_eq!(dashboard.wallet_balance_total, "12");
        assert_eq!(dashboard.wallets.len(), 2);
    }

    #[test]
    fn empty_facts_produce_zero_totals_and_empty_rows() {
        let dashboard =
            organization_reward_dashboard_from_facts(OrganizationRewardDashboardFacts {
                organization_id: 7,
                organization_name: "Empty".to_string(),
                sponsored_teacher_applications: TeacherApplicationDashboardSummaryOutput::default(),
                courses: vec![],
                wallets: vec![],
            });

        assert_eq!(dashboard.course_reward_count, 0);
        assert_eq!(dashboard.approved_reward_count, 0);
        assert_eq!(dashboard.approved_amount_total, "0");
        assert_eq!(dashboard.wallet_balance_total, "0");
        assert!(dashboard.courses.is_empty());
        assert!(dashboard.wallets.is_empty());
    }

    fn course_fact(
        course_id: i32,
        reward_candidate_count: i64,
        approved_reward_count: i64,
        approved_amount_total: BigDecimal,
    ) -> OrganizationCourseRewardDashboardFact {
        OrganizationCourseRewardDashboardFact {
            row: OrganizationCourseRewardDashboardRowOutput {
                course_id,
                course_title: format!("Course {course_id}"),
                reward_candidate_count,
                approved_reward_count,
                approved_amount_total: approved_amount_total.to_string(),
            },
            approved_amount_total,
        }
    }

    fn wallet_fact(wallet_id: i32, balance: BigDecimal) -> OrganizationWalletBalanceFact {
        OrganizationWalletBalanceFact {
            row: OrganizationWalletBalanceRowOutput {
                wallet_id,
                balance: balance.to_string(),
            },
            balance,
        }
    }
}
