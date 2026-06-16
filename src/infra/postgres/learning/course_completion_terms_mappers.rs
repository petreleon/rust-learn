use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsAuditEventOutput, CourseCompletionTermsError, CourseCompletionTermsOutput,
};
use crate::domain::learning::course::{
    CourseCompletionTermsAuditEventType, CourseCompletionTermsStatus,
};
use crate::infra::postgres::models::course_completion_term_audit_event::{
    CourseCompletionTermAuditEvent, NewCourseCompletionTermAuditEvent,
};
use crate::infra::postgres::models::course_completion_terms::{
    CourseCompletionTerms, NewCourseCompletionTerms,
};

impl TryFrom<CourseCompletionTerms> for CourseCompletionTermsOutput {
    type Error = CourseCompletionTermsError;

    fn try_from(row: CourseCompletionTerms) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            course_id: row.course_id,
            teacher_user_id: row.teacher_user_id,
            organization_id: row.organization_id,
            version: row.version,
            status: parse_status(&row.status)?,
            completion_reward_amount: row.completion_reward_amount,
            max_enrolled_students: row.max_enrolled_students,
            reward_policy_id: row.reward_policy_id,
            proposed_by_user_id: row.proposed_by_user_id,
            accepted_by_user_id: row.accepted_by_user_id,
            accepted_at: row.accepted_at,
            activated_at: row.activated_at,
            superseded_at: row.superseded_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

impl From<diesel::result::Error> for CourseCompletionTermsError {
    fn from(error: diesel::result::Error) -> Self {
        map_terms_error(error)
    }
}

impl TryFrom<CourseCompletionTermAuditEvent> for CourseCompletionTermsAuditEventOutput {
    type Error = CourseCompletionTermsError;

    fn try_from(row: CourseCompletionTermAuditEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            terms_id: row.terms_id,
            course_id: row.course_id,
            actor_user_id: row.actor_user_id,
            event_type: parse_event(&row.event_type)?,
            previous_status: row
                .previous_status
                .as_deref()
                .map(parse_status)
                .transpose()?,
            new_status: parse_status(&row.new_status)?,
            completion_reward_amount: row.completion_reward_amount,
            max_enrolled_students: row.max_enrolled_students,
            note: row.note,
            created_at: row.created_at,
        })
    }
}

pub fn new_terms(
    draft: crate::application::learning::manage_course_completion_terms::CourseCompletionTermsDraft,
    version: i32,
) -> NewCourseCompletionTerms {
    NewCourseCompletionTerms {
        course_id: draft.course_id,
        teacher_user_id: draft.teacher_user_id,
        organization_id: draft.organization_id,
        version,
        status: draft.status.as_str().to_string(),
        completion_reward_amount: draft.completion_reward_amount,
        max_enrolled_students: draft.max_enrolled_students,
        proposed_by_user_id: draft.proposed_by_user_id,
    }
}

pub fn new_audit_event(
    terms: &CourseCompletionTerms,
    actor_user_id: i32,
    event_type: CourseCompletionTermsAuditEventType,
    previous_status: Option<CourseCompletionTermsStatus>,
    note: Option<String>,
) -> NewCourseCompletionTermAuditEvent {
    NewCourseCompletionTermAuditEvent {
        terms_id: terms.id,
        course_id: terms.course_id,
        actor_user_id,
        event_type: event_type.as_str().to_string(),
        previous_status: previous_status.map(|status| status.as_str().to_string()),
        new_status: terms.status.clone(),
        completion_reward_amount: terms.completion_reward_amount.clone(),
        max_enrolled_students: terms.max_enrolled_students,
        note,
    }
}

pub fn map_terms_error(error: diesel::result::Error) -> CourseCompletionTermsError {
    match error {
        diesel::result::Error::NotFound => CourseCompletionTermsError::TermsNotFound,
        other => CourseCompletionTermsError::Database(other.to_string()),
    }
}

fn parse_status(value: &str) -> Result<CourseCompletionTermsStatus, CourseCompletionTermsError> {
    CourseCompletionTermsStatus::parse(value)
        .map_err(|error| CourseCompletionTermsError::Database(error.to_string()))
}

fn parse_event(
    value: &str,
) -> Result<CourseCompletionTermsAuditEventType, CourseCompletionTermsError> {
    CourseCompletionTermsAuditEventType::parse(value)
        .map_err(|error| CourseCompletionTermsError::Database(error.to_string()))
}
