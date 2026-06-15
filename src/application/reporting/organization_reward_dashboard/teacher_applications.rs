use crate::application::reporting::organization_reward_dashboard::TeacherApplicationDashboardSummaryOutput;
use crate::domain::teacher_applications::status::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};

pub(crate) fn teacher_application_summary_from_statuses<I, S>(
    statuses: I,
) -> TeacherApplicationDashboardSummaryOutput
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut summary = TeacherApplicationDashboardSummaryOutput::default();
    for status in statuses {
        summary.total += 1;
        match status.as_ref() {
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted += 1,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved += 1,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected += 1,
            _ => {}
        }
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::teacher_application_summary_from_statuses;

    #[test]
    fn buckets_teacher_application_statuses_for_dashboard_summary() {
        let summary = teacher_application_summary_from_statuses([
            "submitted",
            "needs_changes",
            "approved",
            "rejected",
            "approved",
        ]);

        assert_eq!(summary.total, 5);
        assert_eq!(summary.submitted, 1);
        assert_eq!(summary.needs_changes, 1);
        assert_eq!(summary.approved, 2);
        assert_eq!(summary.rejected, 1);
    }

    #[test]
    fn preserves_total_for_unknown_statuses_without_bucket_assignment() {
        let summary = teacher_application_summary_from_statuses(["submitted", "archived"]);

        assert_eq!(summary.total, 2);
        assert_eq!(summary.submitted, 1);
        assert_eq!(summary.needs_changes, 0);
        assert_eq!(summary.approved, 0);
        assert_eq!(summary.rejected, 0);
    }

    #[test]
    fn preserves_exact_status_matching_from_previous_query_behavior() {
        let summary = teacher_application_summary_from_statuses(["submitted", " SUBMITTED "]);

        assert_eq!(summary.total, 2);
        assert_eq!(summary.submitted, 1);
    }
}
