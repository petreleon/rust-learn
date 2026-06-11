pub async fn platform_delegated_permissions_csv(
    conn: &mut AsyncPgConnection,
) -> QueryResult<String> {
    let now = chrono::Utc::now();
    let delegations = delegated_permissions::table
        .order(delegated_permissions::created_at.desc())
        .limit(1000)
        .load::<DelegatedPermission>(conn)
        .await?;

    let mut csv = String::from(
        "delegated_permission_id,grantor_user_id,grantee_user_id,permission,scope_type,organization_id,course_id,state,reason,expires_at,revoked_at,revoked_by_user_id,revoke_reason,created_at,updated_at\n",
    );
    for delegation in delegations {
        let state = if delegation.revoked_at.is_some() {
            "revoked"
        } else if delegation
            .expires_at
            .as_ref()
            .map(|expires_at| *expires_at <= now)
            .unwrap_or(false)
        {
            "expired"
        } else {
            "active"
        };

        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            delegation.id,
            delegation.grantor_user_id,
            delegation.grantee_user_id,
            csv_value(&delegation.permission),
            csv_value(&delegation.scope_type),
            csv_optional(delegation.organization_id),
            csv_optional(delegation.course_id),
            state,
            csv_value(delegation.reason.as_deref().unwrap_or("")),
            csv_optional(delegation.expires_at),
            csv_optional(delegation.revoked_at),
            csv_optional(delegation.revoked_by_user_id),
            csv_value(delegation.revoke_reason.as_deref().unwrap_or("")),
            delegation.created_at,
            delegation.updated_at
        ));
    }

    Ok(csv)
}

pub fn platform_reward_dashboard_csv(dashboard: &PlatformRewardDashboard) -> String {
    let mut csv = String::from("section,metric,value\n");
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

    csv.push_str("\npayout_failures,reward_execution_job_id,reward_candidate_id,status,attempts,last_error,updated_at\n");
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

    csv
}
