use crate::application::reporting::organization_summary::OrganizationSummaryOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OrganizationSummaryFacts {
    pub organization_id: i32,
    pub organization_name: String,
    pub course_ids: Vec<i32>,
    pub member_user_ids: Vec<Option<i32>>,
    pub wallet_count: i64,
    pub course_role_assignment_count: i64,
}

pub(crate) fn organization_summary_from_facts(
    facts: OrganizationSummaryFacts,
) -> OrganizationSummaryOutput {
    OrganizationSummaryOutput {
        organization_id: facts.organization_id,
        organization_name: facts.organization_name,
        course_count: facts.course_ids.len() as i64,
        member_count: facts.member_user_ids.into_iter().flatten().count() as i64,
        wallet_count: facts.wallet_count,
        course_role_assignment_count: facts.course_role_assignment_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_organization_summary_counts_from_facts() {
        let summary = organization_summary_from_facts(OrganizationSummaryFacts {
            organization_id: 42,
            organization_name: "Learning Org".to_string(),
            course_ids: vec![10, 20, 30],
            member_user_ids: vec![Some(1), None, Some(2)],
            wallet_count: 4,
            course_role_assignment_count: 5,
        });

        assert_eq!(summary.organization_id, 42);
        assert_eq!(summary.organization_name, "Learning Org");
        assert_eq!(summary.course_count, 3);
        assert_eq!(summary.member_count, 2);
        assert_eq!(summary.wallet_count, 4);
        assert_eq!(summary.course_role_assignment_count, 5);
    }

    #[test]
    fn empty_facts_produce_zero_counts() {
        let summary = organization_summary_from_facts(OrganizationSummaryFacts {
            organization_id: 7,
            organization_name: "Empty Org".to_string(),
            course_ids: vec![],
            member_user_ids: vec![],
            wallet_count: 0,
            course_role_assignment_count: 0,
        });

        assert_eq!(summary.course_count, 0);
        assert_eq!(summary.member_count, 0);
        assert_eq!(summary.wallet_count, 0);
        assert_eq!(summary.course_role_assignment_count, 0);
    }
}
