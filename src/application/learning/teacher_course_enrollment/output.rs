use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseJoinRequestPageOutput {
    pub requests: Vec<TeacherCourseJoinRequestItemOutput>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseJoinRequestItemOutput {
    pub id: i64,
    pub status: String,
    pub requester: TeacherEnrollmentUserSummaryOutput,
    pub reviewer: Option<TeacherEnrollmentUserSummaryOutput>,
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
    pub can_decide: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseRosterPageOutput {
    pub learners: Vec<TeacherCourseRosterLearnerOutput>,
    pub total: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseRosterLearnerOutput {
    pub user: TeacherEnrollmentUserSummaryOutput,
    pub roles: Vec<String>,
    pub latest_join_request_status: Option<String>,
    pub access_state: String,
    pub can_remove: bool,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
    pub reward_eligibility: TeacherStudentRewardEligibilitySummaryOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherEnrollmentUserSummaryOutput {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseRewardEligibilitySummaryOutput {
    pub supported: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherStudentRewardEligibilitySummaryOutput {
    pub supported: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub reward_candidate_count: i64,
}
