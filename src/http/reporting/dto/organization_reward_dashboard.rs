use serde::Serialize;

use crate::application::reporting::organization_reward_dashboard::{
    OrganizationCourseRewardDashboardRowOutput, OrganizationRewardDashboardOutput,
    OrganizationWalletBalanceRowOutput, TeacherApplicationDashboardSummaryOutput,
};

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationRewardDashboardResponse {
    pub organization_id: i32,
    pub organization_name: String,
    pub sponsored_teacher_applications: TeacherApplicationDashboardSummaryResponse,
    pub course_reward_count: i64,
    pub approved_reward_count: i64,
    pub approved_amount_total: String,
    pub courses: Vec<OrganizationCourseRewardDashboardRowResponse>,
    pub wallets: Vec<OrganizationWalletBalanceRowResponse>,
    pub wallet_balance_total: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TeacherApplicationDashboardSummaryResponse {
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationCourseRewardDashboardRowResponse {
    pub course_id: i32,
    pub course_title: String,
    pub reward_candidate_count: i64,
    pub approved_reward_count: i64,
    pub approved_amount_total: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationWalletBalanceRowResponse {
    pub wallet_id: i32,
    pub balance: String,
}

impl From<OrganizationRewardDashboardOutput> for OrganizationRewardDashboardResponse {
    fn from(output: OrganizationRewardDashboardOutput) -> Self {
        Self {
            organization_id: output.organization_id,
            organization_name: output.organization_name,
            sponsored_teacher_applications: output.sponsored_teacher_applications.into(),
            course_reward_count: output.course_reward_count,
            approved_reward_count: output.approved_reward_count,
            approved_amount_total: output.approved_amount_total,
            courses: output.courses.into_iter().map(Into::into).collect(),
            wallets: output.wallets.into_iter().map(Into::into).collect(),
            wallet_balance_total: output.wallet_balance_total,
        }
    }
}

impl From<TeacherApplicationDashboardSummaryOutput> for TeacherApplicationDashboardSummaryResponse {
    fn from(summary: TeacherApplicationDashboardSummaryOutput) -> Self {
        Self {
            total: summary.total,
            submitted: summary.submitted,
            needs_changes: summary.needs_changes,
            approved: summary.approved,
            rejected: summary.rejected,
        }
    }
}

impl From<OrganizationCourseRewardDashboardRowOutput>
    for OrganizationCourseRewardDashboardRowResponse
{
    fn from(row: OrganizationCourseRewardDashboardRowOutput) -> Self {
        Self {
            course_id: row.course_id,
            course_title: row.course_title,
            reward_candidate_count: row.reward_candidate_count,
            approved_reward_count: row.approved_reward_count,
            approved_amount_total: row.approved_amount_total,
        }
    }
}

impl From<OrganizationWalletBalanceRowOutput> for OrganizationWalletBalanceRowResponse {
    fn from(row: OrganizationWalletBalanceRowOutput) -> Self {
        Self {
            wallet_id: row.wallet_id,
            balance: row.balance,
        }
    }
}
