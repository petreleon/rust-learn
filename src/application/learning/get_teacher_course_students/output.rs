use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardItemOutput;
use crate::application::learning::teacher_course_enrollment::{
    TeacherCourseRewardEligibilitySummaryOutput, TeacherEnrollmentUserSummaryOutput,
    TeacherStudentRewardEligibilitySummaryOutput,
};
use crate::domain::rewards::candidate::event_type::RewardEventType;

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherCourseStudentsOutput {
    pub course: TeacherCourseDashboardItemOutput,
    pub teacher_roles: Vec<String>,
    pub students: Vec<TeacherCourseStudentProgressItemOutput>,
    pub total: i64,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
    pub reward_eligibility: TeacherCourseRewardEligibilitySummaryOutput,
    pub reward_evidence_supported: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherCourseStudentProgressItemOutput {
    pub user: TeacherEnrollmentUserSummaryOutput,
    pub roles: Vec<String>,
    pub access_state: String,
    pub latest_join_request_status: Option<String>,
    pub progress: TeacherStudentProgressSummaryOutput,
    pub reward_eligibility: TeacherStudentRewardEligibilitySummaryOutput,
    pub rewards: TeacherStudentRewardProgressSummaryOutput,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherStudentProgressSummaryOutput {
    pub supported: bool,
    pub completed_content_count: Option<i64>,
    pub total_content_count: usize,
    pub completion_percentage: Option<f64>,
    pub current_content_id: Option<i32>,
    pub current_content_label: Option<String>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherStudentRewardProgressSummaryOutput {
    pub reward_candidate_count: i64,
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub teacher_rejected_count: i64,
    pub completed_count: i64,
    pub failed_count: i64,
    pub latest_candidate: Option<TeacherStudentRewardCandidateSummaryOutput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherStudentRewardCandidateSummaryOutput {
    pub id: i64,
    pub event_type: RewardEventType,
    pub status: String,
    pub evidence: Value,
    pub teacher_decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
