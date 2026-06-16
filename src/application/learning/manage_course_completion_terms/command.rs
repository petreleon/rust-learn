use bigdecimal::BigDecimal;

use crate::domain::learning::course::{
    CourseCompletionTermsAuditEventType, CourseCompletionTermsStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsProposalCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsCounterCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub terms_id: i64,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsDecisionCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub terms_id: i64,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsListQuery {
    pub actor_user_id: i32,
    pub course_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsDraft {
    pub course_id: i32,
    pub teacher_user_id: i32,
    pub organization_id: Option<i32>,
    pub status: CourseCompletionTermsStatus,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub proposed_by_user_id: i32,
    pub previous_terms_id: Option<i64>,
    pub audit_event_type: CourseCompletionTermsAuditEventType,
    pub note: Option<String>,
}
