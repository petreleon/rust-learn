pub fn organization_reward_dashboard_csv(dashboard: &OrganizationRewardDashboard) -> String {
    let mut csv = String::from("section,metric,value\n");
    csv.push_str(&format!(
        "organization,organization_id,{}\n",
        dashboard.organization_id
    ));
    csv.push_str(&format!(
        "organization,organization_name,{}\n",
        csv_value(&dashboard.organization_name)
    ));
    csv.push_str(&format!(
        "teacher_applications,total,{}\n",
        dashboard.sponsored_teacher_applications.total
    ));
    csv.push_str(&format!(
        "teacher_applications,submitted,{}\n",
        dashboard.sponsored_teacher_applications.submitted
    ));
    csv.push_str(&format!(
        "teacher_applications,approved,{}\n",
        dashboard.sponsored_teacher_applications.approved
    ));
    csv.push_str(&format!(
        "reward_candidates,total,{}\n",
        dashboard.course_reward_count
    ));
    csv.push_str(&format!(
        "reward_candidates,approved,{}\n",
        dashboard.approved_reward_count
    ));
    csv.push_str(&format!(
        "reward_candidates,approved_amount_total,{}\n",
        csv_value(&dashboard.approved_amount_total)
    ));
    csv.push_str(&format!(
        "wallets,balance_total,{}\n",
        csv_value(&dashboard.wallet_balance_total)
    ));

    csv.push_str("\ncourses,course_id,course_title,reward_candidate_count,approved_reward_count,approved_amount_total\n");
    for row in &dashboard.courses {
        csv.push_str(&format!(
            "courses,{},{},{},{},{}\n",
            row.course_id,
            csv_value(&row.course_title),
            row.reward_candidate_count,
            row.approved_reward_count,
            csv_value(&row.approved_amount_total)
        ));
    }

    csv.push_str("\nwallets,wallet_id,balance\n");
    for row in &dashboard.wallets {
        csv.push_str(&format!(
            "wallets,{},{}\n",
            row.wallet_id,
            csv_value(&row.balance)
        ));
    }

    csv
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformWalletReconciliationRow {
    pub wallet_id: i32,
    pub owner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub balance: String,
    pub internal_transaction_count: i64,
    pub external_transaction_count: i64,
    pub reward_record_count: i64,
    pub needs_reconciliation_count: i64,
    pub missing_credit_count: i64,
    pub missing_notification_count: i64,
    pub missing_payout_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformWalletReconciliation {
    pub total_wallets: i64,
    pub total_internal_transactions: i64,
    pub total_external_transactions: i64,
    pub total_reward_records: i64,
    pub total_needs_reconciliation: i64,
    pub wallets: Vec<PlatformWalletReconciliationRow>,
}
