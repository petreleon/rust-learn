use crate::application::reporting::platform_csv_exports::PlatformRewardApprovalExportRowOutput;
use crate::http::reporting::dto::csv::{csv_optional, csv_value};

pub fn platform_reward_approvals_csv(rows: &[PlatformRewardApprovalExportRowOutput]) -> String {
    let mut csv = String::from(
        "reward_candidate_id,course_id,student_user_id,submitter_user_id,source_scope,source_organization_id,event_type,status,teacher_approver_user_id,teacher_decision_reason,teacher_decided_at,amount_reviewer_user_id,approved_amount,amount_decision_reason,amount_decided_at,created_at,updated_at\n",
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.reward_candidate_id,
            row.course_id,
            row.student_user_id,
            row.submitter_user_id,
            csv_value(&row.source_scope),
            csv_optional(row.source_organization_id),
            csv_value(&row.event_type),
            csv_value(&row.status),
            csv_optional(row.teacher_approver_user_id),
            csv_value(&row.teacher_decision_reason),
            csv_optional(row.teacher_decided_at.as_ref()),
            csv_optional(row.amount_reviewer_user_id),
            csv_value(&row.approved_amount),
            csv_value(&row.amount_decision_reason),
            csv_optional(row.amount_decided_at.as_ref()),
            row.created_at,
            row.updated_at
        ));
    }
    csv
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::platform_reward_approvals_csv;
    use crate::application::reporting::platform_csv_exports::PlatformRewardApprovalExportRowOutput;

    #[test]
    fn keeps_legacy_reward_approvals_csv_shape() {
        let now = Utc::now();
        let csv = platform_reward_approvals_csv(&[PlatformRewardApprovalExportRowOutput {
            reward_candidate_id: 1,
            course_id: 10,
            student_user_id: 20,
            submitter_user_id: 30,
            source_scope: "course".to_string(),
            source_organization_id: None,
            event_type: "course_completion".to_string(),
            status: "amount_approved".to_string(),
            teacher_approver_user_id: Some(40),
            teacher_decision_reason: "course reward approved".to_string(),
            teacher_decided_at: None,
            amount_reviewer_user_id: Some(50),
            approved_amount: "10".to_string(),
            amount_decision_reason: "platform amount approved".to_string(),
            amount_decided_at: None,
            created_at: now,
            updated_at: now,
        }]);

        assert!(csv.starts_with("reward_candidate_id,course_id"));
        assert!(csv.contains("1,10,20,30,course"));
        assert!(csv.contains("platform amount approved"));
    }
}
