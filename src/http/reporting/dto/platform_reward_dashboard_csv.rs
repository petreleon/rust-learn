use crate::http::reporting::dto::PlatformRewardDashboardResponse;

pub fn platform_reward_dashboard_csv(dashboard: &PlatformRewardDashboardResponse) -> String {
    let mut csv = String::from("section,metric,value\n");
    push_summary_sections(&mut csv, dashboard);
    push_pending_amount_approvals(&mut csv, dashboard);
    push_payout_failures(&mut csv, dashboard);
    push_reconciliation_mismatches(&mut csv, dashboard);
    csv
}

fn push_summary_sections(csv: &mut String, dashboard: &PlatformRewardDashboardResponse) {
    csv.push_str(&format!(
        "teacher_applications,total,{}\n",
        dashboard.teacher_applications.total
    ));
    csv.push_str(&format!(
        "teacher_applications,submitted,{}\n",
        dashboard.teacher_applications.submitted
    ));
    csv.push_str(&format!(
        "teacher_applications,needs_changes,{}\n",
        dashboard.teacher_applications.needs_changes
    ));
    csv.push_str(&format!(
        "teacher_applications,approved,{}\n",
        dashboard.teacher_applications.approved
    ));
    csv.push_str(&format!(
        "teacher_applications,rejected,{}\n",
        dashboard.teacher_applications.rejected
    ));
    csv.push_str(&format!(
        "reward_candidates,total,{}\n",
        dashboard.reward_candidates.total
    ));
    csv.push_str(&format!(
        "reward_candidates,pending_amount_approval,{}\n",
        dashboard.pending_amount_approval_count
    ));
    csv.push_str(&format!(
        "reward_candidates,payout_failures,{}\n",
        dashboard.payout_failure_count
    ));
    csv.push_str(&format!(
        "reward_candidates,reconciliation_mismatches,{}\n",
        dashboard.reconciliation_mismatch_count
    ));
}

fn push_pending_amount_approvals(csv: &mut String, dashboard: &PlatformRewardDashboardResponse) {
    csv.push_str("\npending_amount_approvals,reward_candidate_id,course_id,student_user_id,event_type,status,approved_amount,updated_at\n");
    for row in &dashboard.pending_amount_approvals {
        csv.push_str(&format!(
            "pending_amount_approvals,{},{},{},{},{},{},{}\n",
            row.reward_candidate_id,
            row.course_id,
            row.student_user_id,
            csv_value(&row.event_type),
            csv_value(&row.status),
            csv_value(row.approved_amount.as_deref().unwrap_or("")),
            row.updated_at
        ));
    }
}

fn push_payout_failures(csv: &mut String, dashboard: &PlatformRewardDashboardResponse) {
    csv.push_str(
        "\npayout_failures,reward_execution_job_id,reward_candidate_id,status,attempts,last_error,updated_at\n",
    );
    for row in &dashboard.payout_failures {
        csv.push_str(&format!(
            "payout_failures,{},{},{},{},{},{}\n",
            row.reward_execution_job_id,
            row.reward_candidate_id,
            csv_value(&row.status),
            row.attempts,
            csv_value(row.last_error.as_deref().unwrap_or("")),
            row.updated_at
        ));
    }
}

fn push_reconciliation_mismatches(csv: &mut String, dashboard: &PlatformRewardDashboardResponse) {
    csv.push_str("\nreconciliation_mismatches,reward_candidate_id,course_id,student_user_id,status,mismatch_type,approved_amount,updated_at\n");
    for row in &dashboard.reconciliation_mismatches {
        csv.push_str(&format!(
            "reconciliation_mismatches,{},{},{},{},{},{},{}\n",
            row.reward_candidate_id,
            row.course_id,
            row.student_user_id,
            csv_value(&row.status),
            csv_value(&row.mismatch_type),
            csv_value(row.approved_amount.as_deref().unwrap_or("")),
            row.updated_at
        ));
    }
}

fn csv_value(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::http::reporting::dto::{
        PlatformTeacherApplicationDashboardSummaryResponse, RewardCandidateDashboardRowResponse,
        RewardCandidateDashboardSummaryResponse, RewardExecutionFailureRowResponse,
    };

    #[test]
    fn keeps_legacy_platform_reward_dashboard_csv_shape() {
        let now = Utc::now();
        let csv = platform_reward_dashboard_csv(&PlatformRewardDashboardResponse {
            teacher_applications: PlatformTeacherApplicationDashboardSummaryResponse {
                total: 5,
                submitted: 2,
                needs_changes: 1,
                approved: 1,
                rejected: 1,
            },
            reward_candidates: RewardCandidateDashboardSummaryResponse::default(),
            pending_amount_approval_count: 1,
            pending_amount_approvals: vec![RewardCandidateDashboardRowResponse {
                reward_candidate_id: 1,
                course_id: 10,
                student_user_id: 100,
                submitter_user_id: 200,
                source_organization_id: None,
                event_type: "course_completion".to_string(),
                status: "teacher_approved".to_string(),
                approved_amount: Some("50".to_string()),
                updated_at: now,
            }],
            payout_failure_count: 0,
            payout_failures: vec![RewardExecutionFailureRowResponse {
                reward_execution_job_id: 3,
                reward_candidate_id: 1,
                status: "failed".to_string(),
                attempts: 2,
                last_error: Some("bad, tx".to_string()),
                updated_at: now,
            }],
            reconciliation_mismatch_count: 0,
            reconciliation_mismatches: vec![],
        });

        assert!(csv.starts_with("section,metric,value\n"));
        assert!(csv.contains("teacher_applications,total,5"));
        assert!(csv.contains("pending_amount_approvals,reward_candidate_id"));
        assert!(csv.contains("pending_amount_approvals,1,10,100,"));
        assert!(csv.contains("payout_failures,3,1,failed,2,\"bad, tx\""));
    }
}
