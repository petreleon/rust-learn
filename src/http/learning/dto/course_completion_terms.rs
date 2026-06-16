use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsAuditEventOutput, CourseCompletionTermsCounterCommand,
    CourseCompletionTermsDecisionCommand, CourseCompletionTermsHistoryOutput,
    CourseCompletionTermsOutput, CourseCompletionTermsProposalCommand,
};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CourseCompletionTermsProposalRequest {
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CourseCompletionTermsDecisionRequest {
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseCompletionTermsHistoryResponse {
    pub active_terms: Option<CourseCompletionTermsResponse>,
    pub terms: Vec<CourseCompletionTermsResponse>,
    pub audit_events: Vec<CourseCompletionTermsAuditEventResponse>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseCompletionTermsResponse {
    pub id: i64,
    pub course_id: i32,
    pub teacher_user_id: i32,
    pub organization_id: Option<i32>,
    pub version: i32,
    pub status: String,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub reward_policy_id: Option<i64>,
    pub proposed_by_user_id: i32,
    pub accepted_by_user_id: Option<i32>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub activated_at: Option<DateTime<Utc>>,
    pub superseded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseCompletionTermsAuditEventResponse {
    pub id: i64,
    pub terms_id: i64,
    pub course_id: i32,
    pub actor_user_id: i32,
    pub event_type: String,
    pub previous_status: Option<String>,
    pub new_status: String,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl CourseCompletionTermsProposalRequest {
    pub(in crate::http::learning) fn into_proposal_command(
        self,
        actor_user_id: i32,
        course_id: i32,
    ) -> CourseCompletionTermsProposalCommand {
        CourseCompletionTermsProposalCommand {
            actor_user_id,
            course_id,
            completion_reward_amount: self.completion_reward_amount,
            max_enrolled_students: self.max_enrolled_students,
            note: self.note,
        }
    }

    pub(in crate::http::learning) fn into_counter_command(
        self,
        actor_user_id: i32,
        course_id: i32,
        terms_id: i64,
    ) -> CourseCompletionTermsCounterCommand {
        CourseCompletionTermsCounterCommand {
            actor_user_id,
            course_id,
            terms_id,
            completion_reward_amount: self.completion_reward_amount,
            max_enrolled_students: self.max_enrolled_students,
            note: self.note,
        }
    }
}

impl CourseCompletionTermsDecisionRequest {
    pub(in crate::http::learning) fn into_command(
        self,
        actor_user_id: i32,
        course_id: i32,
        terms_id: i64,
    ) -> CourseCompletionTermsDecisionCommand {
        CourseCompletionTermsDecisionCommand {
            actor_user_id,
            course_id,
            terms_id,
            note: self.note,
        }
    }
}

impl From<CourseCompletionTermsHistoryOutput> for CourseCompletionTermsHistoryResponse {
    fn from(output: CourseCompletionTermsHistoryOutput) -> Self {
        Self {
            active_terms: output.active_terms.map(Into::into),
            terms: output.terms.into_iter().map(Into::into).collect(),
            audit_events: output.audit_events.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CourseCompletionTermsOutput> for CourseCompletionTermsResponse {
    fn from(output: CourseCompletionTermsOutput) -> Self {
        Self {
            id: output.id,
            course_id: output.course_id,
            teacher_user_id: output.teacher_user_id,
            organization_id: output.organization_id,
            version: output.version,
            status: output.status.as_str().to_string(),
            completion_reward_amount: output.completion_reward_amount,
            max_enrolled_students: output.max_enrolled_students,
            reward_policy_id: output.reward_policy_id,
            proposed_by_user_id: output.proposed_by_user_id,
            accepted_by_user_id: output.accepted_by_user_id,
            accepted_at: output.accepted_at,
            activated_at: output.activated_at,
            superseded_at: output.superseded_at,
            created_at: output.created_at,
            updated_at: output.updated_at,
        }
    }
}

impl From<CourseCompletionTermsAuditEventOutput> for CourseCompletionTermsAuditEventResponse {
    fn from(output: CourseCompletionTermsAuditEventOutput) -> Self {
        Self {
            id: output.id,
            terms_id: output.terms_id,
            course_id: output.course_id,
            actor_user_id: output.actor_user_id,
            event_type: output.event_type.as_str().to_string(),
            previous_status: output
                .previous_status
                .map(|status| status.as_str().to_string()),
            new_status: output.new_status.as_str().to_string(),
            completion_reward_amount: output.completion_reward_amount,
            max_enrolled_students: output.max_enrolled_students,
            note: output.note,
            created_at: output.created_at,
        }
    }
}
