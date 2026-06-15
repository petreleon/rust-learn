use crate::application::reporting::platform_summary::PlatformSummaryOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlatformSummaryFact {
    pub total_users: i64,
    pub total_organizations: i64,
    pub total_courses: i64,
    pub total_wallets: i64,
    pub total_notifications: i64,
}

pub(crate) fn platform_summary_output(fact: PlatformSummaryFact) -> PlatformSummaryOutput {
    PlatformSummaryOutput {
        total_users: fact.total_users,
        total_organizations: fact.total_organizations,
        total_courses: fact.total_courses,
        total_wallets: fact.total_wallets,
        total_notifications: fact.total_notifications,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_platform_summary_from_counts() {
        let output = platform_summary_output(PlatformSummaryFact {
            total_users: 1,
            total_organizations: 2,
            total_courses: 3,
            total_wallets: 4,
            total_notifications: 5,
        });

        assert_eq!(output.total_users, 1);
        assert_eq!(output.total_notifications, 5);
    }
}
