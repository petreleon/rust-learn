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

// ── organization_reward_dashboard_csv ──

fn org_dashboard() -> OrganizationRewardDashboard {
    OrganizationRewardDashboard {
        organization_id: 42,
        organization_name: "My Org".into(),
        sponsored_teacher_applications: TeacherApplicationDashboardSummary {
            total: 3,
            submitted: 1,
            needs_changes: 0,
            approved: 1,
            rejected: 1,
        },
        course_reward_count: 10,
        approved_reward_count: 5,
        approved_amount_total: "500".into(),
        courses: vec![OrganizationCourseRewardDashboardRow {
            course_id: 1,
            course_title: "Rust 101".into(),
            reward_candidate_count: 3,
            approved_reward_count: 2,
            approved_amount_total: "200".into(),
        }],
        wallets: vec![OrganizationWalletBalanceRow {
            wallet_id: 1,
            balance: "300".into(),
        }],
        wallet_balance_total: "300".into(),
    }
}

#[test]
fn org_reward_dashboard_csv_has_sections() {
    let csv = organization_reward_dashboard_csv(&org_dashboard());
    assert!(csv.contains("organization,organization_id,42"));
    assert!(csv.contains("organization,organization_name,My Org"));
    assert!(csv.contains("teacher_applications,total,3"));
    assert!(csv.contains("reward_candidates,total,10"));
    assert!(csv.contains("reward_candidates,approved,5"));
    assert!(csv.contains("courses,course_id,"));
    assert!(csv.contains("courses,1,Rust 101"));
    assert!(csv.contains("wallets,wallet_id,"));
    assert!(csv.contains("wallets,1,300"));
}
