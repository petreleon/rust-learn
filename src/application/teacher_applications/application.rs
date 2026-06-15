use chrono::{DateTime, Utc};

use crate::domain::teacher_applications::portfolio::TeacherApplicationPortfolioLinks;

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherApplicationOutput {
    pub id: i64,
    pub applicant_user_id: i32,
    pub requested_scope: String,
    pub requested_organization_id: Option<i32>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub organization_sponsor_id: Option<i32>,
    pub portfolio_links: TeacherApplicationPortfolioLinks,
    pub status: String,
    pub reviewer_id: Option<i32>,
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
    pub idempotency_key: Option<String>,
}
