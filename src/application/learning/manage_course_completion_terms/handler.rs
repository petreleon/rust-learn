use crate::application::learning::manage_course_completion_terms::validation::{
    ensure_can_decide, ensure_can_propose, ensure_can_view, ensure_open_terms,
    validate_terms_values,
};
use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsCounterCommand, CourseCompletionTermsDecisionCommand,
    CourseCompletionTermsDraft, CourseCompletionTermsError, CourseCompletionTermsHistoryOutput,
    CourseCompletionTermsListQuery, CourseCompletionTermsOutput,
    CourseCompletionTermsProposalCommand, CourseCompletionTermsStore,
};
use crate::domain::learning::course::{
    CourseCompletionTermsAuditEventType, CourseCompletionTermsStatus,
};

pub async fn list_course_completion_terms(
    store: &mut impl CourseCompletionTermsStore,
    query: CourseCompletionTermsListQuery,
) -> Result<CourseCompletionTermsHistoryOutput, CourseCompletionTermsError> {
    store.course_context(query.course_id).await?;
    ensure_can_view(store, query.actor_user_id, query.course_id).await?;

    Ok(CourseCompletionTermsHistoryOutput {
        active_terms: store.active_terms(query.course_id).await?,
        terms: store.list_terms(query.course_id).await?,
        audit_events: store.list_audit_events(query.course_id).await?,
    })
}

pub async fn submit_course_completion_terms(
    store: &mut impl CourseCompletionTermsStore,
    command: CourseCompletionTermsProposalCommand,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    validate_terms_values(
        &command.completion_reward_amount,
        command.max_enrolled_students,
    )?;
    let context = store.course_context(command.course_id).await?;
    ensure_can_propose(store, command.actor_user_id, command.course_id).await?;

    store
        .create_terms(CourseCompletionTermsDraft {
            course_id: context.course_id,
            teacher_user_id: command.actor_user_id,
            organization_id: context.organization_id,
            status: CourseCompletionTermsStatus::Submitted,
            completion_reward_amount: command.completion_reward_amount,
            max_enrolled_students: command.max_enrolled_students,
            proposed_by_user_id: command.actor_user_id,
            previous_terms_id: None,
            audit_event_type: CourseCompletionTermsAuditEventType::Proposed,
            note: command.note,
        })
        .await
}

pub async fn counter_course_completion_terms(
    store: &mut impl CourseCompletionTermsStore,
    command: CourseCompletionTermsCounterCommand,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    validate_terms_values(
        &command.completion_reward_amount,
        command.max_enrolled_students,
    )?;
    let existing = open_terms(store, command.course_id, command.terms_id).await?;
    ensure_can_decide(store, command.actor_user_id, command.course_id).await?;

    store
        .create_terms(CourseCompletionTermsDraft {
            course_id: command.course_id,
            teacher_user_id: existing.teacher_user_id,
            organization_id: existing.organization_id,
            status: CourseCompletionTermsStatus::Countered,
            completion_reward_amount: command.completion_reward_amount,
            max_enrolled_students: command.max_enrolled_students,
            proposed_by_user_id: command.actor_user_id,
            previous_terms_id: Some(existing.id),
            audit_event_type: CourseCompletionTermsAuditEventType::Countered,
            note: command.note,
        })
        .await
}

pub async fn accept_course_completion_terms(
    store: &mut impl CourseCompletionTermsStore,
    command: CourseCompletionTermsDecisionCommand,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    open_terms(store, command.course_id, command.terms_id).await?;
    ensure_can_decide(store, command.actor_user_id, command.course_id).await?;
    store
        .activate_terms(command.terms_id, command.actor_user_id, command.note)
        .await
}

pub async fn reject_course_completion_terms(
    store: &mut impl CourseCompletionTermsStore,
    command: CourseCompletionTermsDecisionCommand,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    open_terms(store, command.course_id, command.terms_id).await?;
    ensure_can_decide(store, command.actor_user_id, command.course_id).await?;
    store
        .reject_terms(command.terms_id, command.actor_user_id, command.note)
        .await
}

pub async fn withdraw_course_completion_terms(
    store: &mut impl CourseCompletionTermsStore,
    command: CourseCompletionTermsDecisionCommand,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    let terms = open_terms(store, command.course_id, command.terms_id).await?;
    if command.actor_user_id != terms.teacher_user_id {
        ensure_can_propose(store, command.actor_user_id, command.course_id).await?;
    }
    store
        .withdraw_terms(command.terms_id, command.actor_user_id, command.note)
        .await
}

async fn open_terms(
    store: &mut impl CourseCompletionTermsStore,
    course_id: i32,
    terms_id: i64,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    let terms = store
        .terms_by_id(course_id, terms_id)
        .await?
        .ok_or(CourseCompletionTermsError::TermsNotFound)?;
    ensure_open_terms(&terms)?;
    Ok(terms)
}
