use crate::application::organizations::list_organization_courses::OrganizationCourseRewardQueueSummaryOutput;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub(crate) fn record_organization_course_reward_queue_status(
    summary: &mut OrganizationCourseRewardQueueSummaryOutput,
    status: RewardCandidateStatus,
    count: i64,
) {
    match status {
        RewardCandidateStatus::PendingTeacherApproval => {
            summary.pending_teacher_count += count;
        }
        RewardCandidateStatus::TeacherApproved => {
            summary.teacher_approved_count += count;
        }
        RewardCandidateStatus::Failed => {
            summary.failed_count += count;
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus as Status;

    fn summary() -> OrganizationCourseRewardQueueSummaryOutput {
        OrganizationCourseRewardQueueSummaryOutput {
            pending_teacher_count: 0,
            teacher_approved_count: 0,
            failed_count: 0,
        }
    }

    #[test]
    fn records_reward_queue_status_counts_for_course_list() {
        let mut summary = summary();

        record_organization_course_reward_queue_status(
            &mut summary,
            Status::PendingTeacherApproval,
            2,
        );
        record_organization_course_reward_queue_status(&mut summary, Status::TeacherApproved, 3);
        record_organization_course_reward_queue_status(&mut summary, Status::Failed, 1);

        assert_eq!(summary.pending_teacher_count, 2);
        assert_eq!(summary.teacher_approved_count, 3);
        assert_eq!(summary.failed_count, 1);
    }

    #[test]
    fn ignores_statuses_outside_the_course_reward_queue() {
        let mut output = summary();

        for status in [
            Status::AmountApproved,
            Status::WalletCredited,
            Status::Completed,
            Status::NeedsReconciliation,
        ] {
            record_organization_course_reward_queue_status(&mut output, status, 4);
        }

        assert_eq!(output, summary());
    }
}
