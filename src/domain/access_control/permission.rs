#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    ApproveRewardAmount,
    ApproveStudentRewardCandidate,
    CreateRewardableCourseEvent,
    CreateWallet,
    ExecuteRewardPayout,
    ManageCourseRewardRules,
    ManageOrgRewardBudget,
    ManageOrgWallets,
    ManageWallets,
    ReconcileWallets,
    SetRewardPolicy,
    SetDepositTax,
    SetRetireTax,
    SubmitCourseRewardEvent,
    SubmitOrgCourseRewardEvent,
    ViewCourseRewardStatus,
    ViewRewardAudit,
    ViewOrgRewardReports,
    ViewTransactions,
    ViewWallet,
}

impl Permission {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApproveRewardAmount => "APPROVE_REWARD_AMOUNT",
            Self::ApproveStudentRewardCandidate => "APPROVE_STUDENT_REWARD_CANDIDATE",
            Self::CreateRewardableCourseEvent => "CREATE_REWARDABLE_COURSE_EVENT",
            Self::CreateWallet => "CREATE_WALLET",
            Self::ExecuteRewardPayout => "EXECUTE_REWARD_PAYOUT",
            Self::ManageCourseRewardRules => "MANAGE_COURSE_REWARD_RULES",
            Self::ManageOrgRewardBudget => "MANAGE_ORG_REWARD_BUDGET",
            Self::ManageOrgWallets => "MANAGE_ORG_WALLETS",
            Self::ManageWallets => "MANAGE_WALLETS",
            Self::ReconcileWallets => "RECONCILE_WALLETS",
            Self::SetRewardPolicy => "SET_REWARD_POLICY",
            Self::SetDepositTax => "SET_DEPOSIT_TAX",
            Self::SetRetireTax => "SET_RETIRE_TAX",
            Self::SubmitCourseRewardEvent => "SUBMIT_COURSE_REWARD_EVENT",
            Self::SubmitOrgCourseRewardEvent => "SUBMIT_ORG_COURSE_REWARD_EVENT",
            Self::ViewCourseRewardStatus => "VIEW_COURSE_REWARD_STATUS",
            Self::ViewRewardAudit => "VIEW_REWARD_AUDIT",
            Self::ViewOrgRewardReports => "VIEW_ORG_REWARD_REPORTS",
            Self::ViewTransactions => "VIEW_TRANSACTIONS",
            Self::ViewWallet => "VIEW_WALLET",
        }
    }
}
