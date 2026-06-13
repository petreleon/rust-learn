#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardAuthorizationAction {
    ApproveRewardAmount,
    ApproveStudentRewardCandidate { course_id: i32 },
    ExecuteRewardPayout,
    ManageCourseRewardRules { course_id: i32 },
    ManageOrganizationRewardFraudBlock,
    ManageRewardPolicy,
    ManageRewardFraudBlock,
    ManageTeacherRewardFraudBlock,
    RecordRewardCompensation,
    SubmitCourseRewardEvent { course_id: i32 },
    SubmitOrganizationCourseRewardEvent { organization_id: i32 },
    ViewCourseRewardStatus { course_id: i32 },
    ViewRewardFraudBlocks,
    ViewRewardAudit,
}
