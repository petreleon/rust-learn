use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use crate::application::learning::get_teacher_course_students::{
    TeacherCourseStudentProgressItemOutput, TeacherCourseStudentsOutput,
    TeacherStudentProgressSummaryOutput, TeacherStudentRewardCandidateSummaryOutput,
    TeacherStudentRewardProgressSummaryOutput,
};

use super::teacher_course_dashboard::TeacherCourseDashboardItemResponse;
use super::teacher_course_enrollment::{
    TeacherCourseRewardEligibilitySummaryResponse, TeacherEnrollmentUserSummaryResponse,
    TeacherStudentRewardEligibilitySummaryResponse,
};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TeacherCourseStudentsResponse {
    pub course: TeacherCourseDashboardItemResponse,
    pub teacher_roles: Vec<String>,
    pub students: Vec<TeacherCourseStudentProgressItemResponse>,
    pub total: i64,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
    pub reward_eligibility: TeacherCourseRewardEligibilitySummaryResponse,
    pub reward_evidence_supported: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TeacherCourseStudentProgressItemResponse {
    pub user: TeacherEnrollmentUserSummaryResponse,
    pub roles: Vec<String>,
    pub access_state: String,
    pub latest_join_request_status: Option<String>,
    pub progress: TeacherStudentProgressSummaryResponse,
    pub reward_eligibility: TeacherStudentRewardEligibilitySummaryResponse,
    pub rewards: TeacherStudentRewardProgressSummaryResponse,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TeacherStudentProgressSummaryResponse {
    pub supported: bool,
    pub completed_content_count: Option<i64>,
    pub total_content_count: usize,
    pub completion_percentage: Option<f64>,
    pub current_content_id: Option<i32>,
    pub current_content_label: Option<String>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TeacherStudentRewardProgressSummaryResponse {
    pub reward_candidate_count: i64,
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub teacher_rejected_count: i64,
    pub completed_count: i64,
    pub failed_count: i64,
    pub latest_candidate: Option<TeacherStudentRewardCandidateSummaryResponse>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TeacherStudentRewardCandidateSummaryResponse {
    pub id: i64,
    pub event_type: String,
    pub status: String,
    pub evidence: Value,
    pub teacher_decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TeacherCourseStudentsOutput> for TeacherCourseStudentsResponse {
    fn from(output: TeacherCourseStudentsOutput) -> Self {
        Self {
            course: output.course.into(),
            teacher_roles: output.teacher_roles,
            students: output.students.into_iter().map(Into::into).collect(),
            total: output.total,
            progress_supported: output.progress_supported,
            reward_eligibility_supported: output.reward_eligibility_supported,
            reward_eligibility: output.reward_eligibility.into(),
            reward_evidence_supported: output.reward_evidence_supported,
        }
    }
}

impl From<TeacherCourseStudentProgressItemOutput> for TeacherCourseStudentProgressItemResponse {
    fn from(student: TeacherCourseStudentProgressItemOutput) -> Self {
        Self {
            user: student.user.into(),
            roles: student.roles,
            access_state: student.access_state,
            latest_join_request_status: student.latest_join_request_status,
            progress: student.progress.into(),
            reward_eligibility: student.reward_eligibility.into(),
            rewards: student.rewards.into(),
        }
    }
}

impl From<TeacherStudentProgressSummaryOutput> for TeacherStudentProgressSummaryResponse {
    fn from(progress: TeacherStudentProgressSummaryOutput) -> Self {
        Self {
            supported: progress.supported,
            completed_content_count: progress.completed_content_count,
            total_content_count: progress.total_content_count,
            completion_percentage: progress.completion_percentage,
            current_content_id: progress.current_content_id,
            current_content_label: progress.current_content_label,
            last_activity_at: progress.last_activity_at,
            note: progress.note,
        }
    }
}

impl From<TeacherStudentRewardProgressSummaryOutput>
    for TeacherStudentRewardProgressSummaryResponse
{
    fn from(rewards: TeacherStudentRewardProgressSummaryOutput) -> Self {
        Self {
            reward_candidate_count: rewards.reward_candidate_count,
            pending_teacher_count: rewards.pending_teacher_count,
            teacher_approved_count: rewards.teacher_approved_count,
            teacher_rejected_count: rewards.teacher_rejected_count,
            completed_count: rewards.completed_count,
            failed_count: rewards.failed_count,
            latest_candidate: rewards.latest_candidate.map(Into::into),
        }
    }
}

impl From<TeacherStudentRewardCandidateSummaryOutput>
    for TeacherStudentRewardCandidateSummaryResponse
{
    fn from(candidate: TeacherStudentRewardCandidateSummaryOutput) -> Self {
        Self {
            id: candidate.id,
            event_type: candidate.event_type.as_str().to_string(),
            status: candidate.status,
            evidence: candidate.evidence,
            teacher_decision_reason: candidate.teacher_decision_reason,
            created_at: candidate.created_at,
            updated_at: candidate.updated_at,
        }
    }
}
