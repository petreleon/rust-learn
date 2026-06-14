#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationRewardDashboardOutput {
    pub organization_id: i32,
    pub organization_name: String,
    pub sponsored_teacher_applications: TeacherApplicationDashboardSummaryOutput,
    pub course_reward_count: i64,
    pub approved_reward_count: i64,
    pub approved_amount_total: String,
    pub courses: Vec<OrganizationCourseRewardDashboardRowOutput>,
    pub wallets: Vec<OrganizationWalletBalanceRowOutput>,
    pub wallet_balance_total: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationDashboardSummaryOutput {
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseRewardDashboardRowOutput {
    pub course_id: i32,
    pub course_title: String,
    pub reward_candidate_count: i64,
    pub approved_reward_count: i64,
    pub approved_amount_total: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationWalletBalanceRowOutput {
    pub wallet_id: i32,
    pub balance: String,
}
