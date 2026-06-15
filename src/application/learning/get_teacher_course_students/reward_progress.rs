use crate::application::learning::get_teacher_course_students::TeacherStudentRewardProgressSummaryOutput;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub(crate) fn record_teacher_student_reward_progress_status(
    summary: &mut TeacherStudentRewardProgressSummaryOutput,
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
        RewardCandidateStatus::TeacherRejected => {
            summary.teacher_rejected_count += count;
        }
        RewardCandidateStatus::Completed => {
            summary.completed_count += count;
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

    fn summary() -> TeacherStudentRewardProgressSummaryOutput {
        TeacherStudentRewardProgressSummaryOutput {
            reward_candidate_count: 8,
            pending_teacher_count: 0,
            teacher_approved_count: 0,
            teacher_rejected_count: 0,
            completed_count: 0,
            failed_count: 0,
            latest_candidate: None,
        }
    }

    #[test]
    fn records_student_reward_progress_status_counts() {
        let mut summary = summary();

        record_teacher_student_reward_progress_status(
            &mut summary,
            Status::PendingTeacherApproval,
            2,
        );
        record_teacher_student_reward_progress_status(&mut summary, Status::TeacherApproved, 3);
        record_teacher_student_reward_progress_status(&mut summary, Status::TeacherRejected, 1);
        record_teacher_student_reward_progress_status(&mut summary, Status::Completed, 4);
        record_teacher_student_reward_progress_status(&mut summary, Status::Failed, 5);

        assert_eq!(summary.pending_teacher_count, 2);
        assert_eq!(summary.teacher_approved_count, 3);
        assert_eq!(summary.teacher_rejected_count, 1);
        assert_eq!(summary.completed_count, 4);
        assert_eq!(summary.failed_count, 5);
    }

    #[test]
    fn ignores_statuses_outside_student_reward_progress_buckets() {
        let mut output = summary();

        for status in [
            Status::AmountApproved,
            Status::WalletCredited,
            Status::Notified,
            Status::NeedsReconciliation,
        ] {
            record_teacher_student_reward_progress_status(&mut output, status, 4);
        }

        assert_eq!(output, summary());
    }
}
