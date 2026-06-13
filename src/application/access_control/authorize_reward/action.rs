#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardAuthorizationAction {
    ApproveRewardAmount,
    ExecuteRewardPayout,
    ManageRewardPolicy,
    RecordRewardCompensation,
    ViewRewardAudit,
}
