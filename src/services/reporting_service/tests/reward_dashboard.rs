// ── platform_reward_dashboard_csv ──

fn reward_dashboard() -> PlatformRewardDashboard {
    PlatformRewardDashboard {
        teacher_applications: TeacherApplicationDashboardSummary {
            total: 5,
            submitted: 2,
            needs_changes: 1,
            approved: 1,
            rejected: 1,
        },
        reward_candidates: RewardCandidateDashboardSummary::default(),
        pending_amount_approvals: vec![RewardCandidateDashboardRow {
            reward_candidate_id: 1,
            course_id: 10,
            student_user_id: 100,
            submitter_user_id: 200,
            source_organization_id: None,
            event_type: "course_completion".into(),
            status: "teacher_approved".into(),
            approved_amount: Some("50".into()),
            updated_at: Utc::now(),
        }],
        pending_amount_approval_count: 1,
        payout_failures: vec![],
        payout_failure_count: 0,
        reconciliation_mismatches: vec![],
        reconciliation_mismatch_count: 0,
    }
}

#[test]
fn reward_dashboard_csv_has_sections() {
    let csv = platform_reward_dashboard_csv(&reward_dashboard());
    assert!(csv.contains("teacher_applications,total,5"));
    assert!(csv.contains("teacher_applications,submitted,2"));
    assert!(csv.contains("pending_amount_approvals,reward_candidate_id"));
    assert!(csv.contains("pending_amount_approvals,1,10,100,"));
}
