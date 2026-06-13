use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::learning::course_enrollment::{
    CourseEnrollmentRemovalOutput, CourseJoinRequestOutput, DecideCourseJoinCommand,
};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CourseJoinDecisionRequest {
    pub status: String,
    pub decision_reason: Option<String>,
}

impl CourseJoinDecisionRequest {
    pub fn into_command(
        self,
        reviewer_user_id: i32,
        course_id: i32,
        request_id: i64,
    ) -> DecideCourseJoinCommand {
        DecideCourseJoinCommand {
            reviewer_user_id,
            course_id,
            request_id,
            status: self.status,
            decision_reason: self.decision_reason,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseJoinRequestResponse {
    pub id: i64,
    pub course_id: i32,
    pub requester_user_id: i32,
    pub status: String,
    pub reviewer_user_id: Option<i32>,
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
}

impl From<CourseJoinRequestOutput> for CourseJoinRequestResponse {
    fn from(request: CourseJoinRequestOutput) -> Self {
        Self {
            id: request.id,
            course_id: request.course_id,
            requester_user_id: request.requester_user_id,
            status: request.status,
            reviewer_user_id: request.reviewer_user_id,
            decision_reason: request.decision_reason,
            created_at: request.created_at,
            updated_at: request.updated_at,
            decided_at: request.decided_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseEnrollmentRemovalResponse {
    pub course_id: i32,
    pub user_id: i32,
    pub removed: bool,
}

impl From<CourseEnrollmentRemovalOutput> for CourseEnrollmentRemovalResponse {
    fn from(removal: CourseEnrollmentRemovalOutput) -> Self {
        Self {
            course_id: removal.course_id,
            user_id: removal.user_id,
            removed: removal.removed,
        }
    }
}
