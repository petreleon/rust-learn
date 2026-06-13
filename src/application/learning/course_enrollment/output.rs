use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseJoinRequestOutput {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseJoinDecisionOutput {
    pub join_request: CourseJoinRequestOutput,
    pub enrollment_notification: Option<EnrollmentNotification>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnrollmentNotification {
    pub target_user_id: i32,
    pub course_id: i32,
    pub course_title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseEnrollmentRemovalOutput {
    pub course_id: i32,
    pub user_id: i32,
    pub removed: bool,
}
