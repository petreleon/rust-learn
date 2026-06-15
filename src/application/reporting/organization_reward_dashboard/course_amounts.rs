use bigdecimal::BigDecimal;

use crate::application::reporting::organization_reward_dashboard::OrganizationCourseRewardDashboardFact;

pub(crate) fn organization_course_reward_fact_from_amounts<I>(
    course_id: i32,
    course_title: String,
    amounts: I,
) -> OrganizationCourseRewardDashboardFact
where
    I: IntoIterator<Item = Option<BigDecimal>>,
{
    let mut reward_candidate_count = 0;
    let mut approved_reward_count = 0;
    let mut approved_amount_total = BigDecimal::from(0);

    for amount in amounts {
        reward_candidate_count += 1;
        if let Some(amount) = amount {
            approved_reward_count += 1;
            approved_amount_total += amount;
        }
    }

    OrganizationCourseRewardDashboardFact::new(
        course_id,
        course_title,
        reward_candidate_count,
        approved_reward_count,
        approved_amount_total,
    )
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use super::*;
    use crate::application::reporting::organization_reward_dashboard::{
        organization_reward_dashboard_from_facts, OrganizationRewardDashboardFacts,
        OrganizationRewardDashboardOutput, TeacherApplicationDashboardSummaryOutput,
    };

    #[test]
    fn summarizes_course_reward_amounts_into_dashboard_fact() {
        let fact = organization_course_reward_fact_from_amounts(
            9,
            "Rust Modules".to_string(),
            [Some(BigDecimal::from(12)), None, Some(BigDecimal::from(8))],
        );

        let dashboard = dashboard_from_courses(vec![fact]);

        assert_eq!(dashboard.course_reward_count, 3);
        assert_eq!(dashboard.approved_reward_count, 2);
        assert_eq!(dashboard.approved_amount_total, "20");
        assert_eq!(dashboard.courses[0].course_id, 9);
        assert_eq!(dashboard.courses[0].course_title, "Rust Modules");
        assert_eq!(dashboard.courses[0].approved_amount_total, "20");
    }

    #[test]
    fn empty_course_amounts_produce_zero_counts_and_total() {
        let fact = organization_course_reward_fact_from_amounts(
            5,
            "No Rewards".to_string(),
            Vec::<Option<BigDecimal>>::new(),
        );

        let dashboard = dashboard_from_courses(vec![fact]);

        assert_eq!(dashboard.course_reward_count, 0);
        assert_eq!(dashboard.approved_reward_count, 0);
        assert_eq!(dashboard.approved_amount_total, "0");
        assert_eq!(dashboard.courses[0].approved_amount_total, "0");
    }

    fn dashboard_from_courses(
        courses: Vec<OrganizationCourseRewardDashboardFact>,
    ) -> OrganizationRewardDashboardOutput {
        organization_reward_dashboard_from_facts(OrganizationRewardDashboardFacts {
            organization_id: 1,
            organization_name: "Org".to_string(),
            sponsored_teacher_applications: TeacherApplicationDashboardSummaryOutput::default(),
            courses,
            wallets: vec![],
        })
    }
}
