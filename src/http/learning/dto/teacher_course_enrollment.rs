use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::learning::teacher_course_enrollment::{
    TeacherCourseJoinRequestItemOutput, TeacherCourseJoinRequestPageOutput,
    TeacherCourseRewardEligibilitySummaryOutput, TeacherCourseRosterLearnerOutput,
    TeacherCourseRosterPageOutput, TeacherEnrollmentUserSummaryOutput,
    TeacherStudentRewardEligibilitySummaryOutput,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseJoinRequestPageResponse {
    pub requests: Vec<TeacherCourseJoinRequestItemResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseJoinRequestItemResponse {
    pub id: i64,
    pub status: String,
    pub requester: TeacherEnrollmentUserSummaryResponse,
    pub reviewer: Option<TeacherEnrollmentUserSummaryResponse>,
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
    pub can_decide: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseRosterPageResponse {
    pub learners: Vec<TeacherCourseRosterLearnerResponse>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseRosterLearnerResponse {
    pub user: TeacherEnrollmentUserSummaryResponse,
    pub roles: Vec<String>,
    pub latest_join_request_status: Option<String>,
    pub access_state: String,
    pub can_remove: bool,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
    pub reward_eligibility: TeacherStudentRewardEligibilitySummaryResponse,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherEnrollmentUserSummaryResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseRewardEligibilitySummaryResponse {
    pub supported: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherStudentRewardEligibilitySummaryResponse {
    pub supported: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub reward_candidate_count: i64,
}

impl From<TeacherCourseJoinRequestPageOutput> for TeacherCourseJoinRequestPageResponse {
    fn from(page: TeacherCourseJoinRequestPageOutput) -> Self {
        Self {
            requests: page.requests.into_iter().map(Into::into).collect(),
            total: page.total,
            limit: page.limit,
            offset: page.offset,
            status: page.status,
        }
    }
}

impl From<TeacherCourseJoinRequestItemOutput> for TeacherCourseJoinRequestItemResponse {
    fn from(request: TeacherCourseJoinRequestItemOutput) -> Self {
        Self {
            id: request.id,
            status: request.status,
            requester: request.requester.into(),
            reviewer: request.reviewer.map(Into::into),
            decision_reason: request.decision_reason,
            created_at: request.created_at,
            updated_at: request.updated_at,
            decided_at: request.decided_at,
            can_decide: request.can_decide,
        }
    }
}

impl From<TeacherCourseRosterPageOutput> for TeacherCourseRosterPageResponse {
    fn from(page: TeacherCourseRosterPageOutput) -> Self {
        Self {
            learners: page.learners.into_iter().map(Into::into).collect(),
            total: page.total,
        }
    }
}

impl From<TeacherCourseRosterLearnerOutput> for TeacherCourseRosterLearnerResponse {
    fn from(learner: TeacherCourseRosterLearnerOutput) -> Self {
        Self {
            user: learner.user.into(),
            roles: learner.roles,
            latest_join_request_status: learner.latest_join_request_status,
            access_state: learner.access_state,
            can_remove: learner.can_remove,
            progress_supported: learner.progress_supported,
            reward_eligibility_supported: learner.reward_eligibility_supported,
            reward_eligibility: learner.reward_eligibility.into(),
        }
    }
}

impl From<TeacherEnrollmentUserSummaryOutput> for TeacherEnrollmentUserSummaryResponse {
    fn from(user: TeacherEnrollmentUserSummaryOutput) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            kyc_verified: user.kyc_verified,
        }
    }
}

impl From<TeacherCourseRewardEligibilitySummaryOutput>
    for TeacherCourseRewardEligibilitySummaryResponse
{
    fn from(eligibility: TeacherCourseRewardEligibilitySummaryOutput) -> Self {
        Self {
            supported: eligibility.supported,
            active_policy_count: eligibility.active_policy_count,
            event_types: eligibility.event_types,
        }
    }
}

impl From<TeacherStudentRewardEligibilitySummaryOutput>
    for TeacherStudentRewardEligibilitySummaryResponse
{
    fn from(eligibility: TeacherStudentRewardEligibilitySummaryOutput) -> Self {
        Self {
            supported: eligibility.supported,
            active_policy_count: eligibility.active_policy_count,
            event_types: eligibility.event_types,
            reward_candidate_count: eligibility.reward_candidate_count,
        }
    }
}
