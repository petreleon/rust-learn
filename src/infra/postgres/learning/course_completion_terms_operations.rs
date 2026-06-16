use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsDraft, CourseCompletionTermsError, CourseCompletionTermsOutput,
};
use crate::domain::learning::course::CourseCompletionTermsAuditEventType;
use crate::infra::postgres::learning::{
    course_completion_terms_mappers, course_completion_terms_records,
    course_completion_terms_reward_policy, course_completion_terms_supersede,
    course_completion_terms_transitions,
};
use crate::infra::postgres::models::course_completion_terms::CourseCompletionTerms;

pub async fn create_terms(
    conn: &mut AsyncPgConnection,
    draft: CourseCompletionTermsDraft,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    conn.transaction::<_, CourseCompletionTermsError, _>(|conn| {
        Box::pin(async move {
            let now = chrono::Utc::now();
            let actor_user_id = draft.proposed_by_user_id;
            let event_type = draft.audit_event_type;
            let note = draft.note.clone();
            for terms in
                course_completion_terms_supersede::supersede_open_terms(conn, draft.course_id, now)
                    .await?
            {
                insert_superseded_audit(conn, &terms, actor_user_id).await?;
            }
            let version =
                course_completion_terms_records::next_version(conn, draft.course_id).await?;
            let terms = course_completion_terms_records::create_terms(
                conn,
                course_completion_terms_mappers::new_terms(draft, version),
            )
            .await?;
            course_completion_terms_records::insert_audit_event(
                conn,
                course_completion_terms_mappers::new_audit_event(
                    &terms,
                    actor_user_id,
                    event_type,
                    None,
                    note,
                ),
            )
            .await?;
            CourseCompletionTermsOutput::try_from(terms)
        })
    })
    .await
}

pub async fn activate_terms(
    conn: &mut AsyncPgConnection,
    terms_id: i64,
    actor_user_id: i32,
    note: Option<String>,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    conn.transaction::<_, CourseCompletionTermsError, _>(|conn| {
        Box::pin(async move {
            let terms = course_completion_terms_records::terms_by_terms_id(conn, terms_id).await?;
            for active in course_completion_terms_supersede::supersede_active_terms(
                conn,
                terms.course_id,
                chrono::Utc::now(),
            )
            .await?
            {
                insert_superseded_audit(conn, &active, actor_user_id).await?;
            }
            let policy_id = course_completion_terms_reward_policy::create_completion_policy(
                conn,
                &terms,
                actor_user_id,
            )
            .await?;
            let updated = course_completion_terms_transitions::mark_active(
                conn,
                terms_id,
                actor_user_id,
                policy_id,
                note,
                chrono::Utc::now(),
            )
            .await?;
            CourseCompletionTermsOutput::try_from(updated)
        })
    })
    .await
}

pub async fn transition_terms(
    conn: &mut AsyncPgConnection,
    terms_id: i64,
    actor_user_id: i32,
    note: Option<String>,
    withdrawn: bool,
) -> Result<CourseCompletionTermsOutput, CourseCompletionTermsError> {
    conn.transaction::<_, CourseCompletionTermsError, _>(|conn| {
        Box::pin(async move {
            let now = chrono::Utc::now();
            let row = if withdrawn {
                course_completion_terms_transitions::mark_withdrawn(
                    conn,
                    terms_id,
                    actor_user_id,
                    note,
                    now,
                )
                .await?
            } else {
                course_completion_terms_transitions::mark_rejected(
                    conn,
                    terms_id,
                    actor_user_id,
                    note,
                    now,
                )
                .await?
            };
            CourseCompletionTermsOutput::try_from(row)
        })
    })
    .await
}

async fn insert_superseded_audit(
    conn: &mut AsyncPgConnection,
    terms: &CourseCompletionTerms,
    actor_user_id: i32,
) -> Result<(), CourseCompletionTermsError> {
    course_completion_terms_records::insert_audit_event(
        conn,
        course_completion_terms_mappers::new_audit_event(
            terms,
            actor_user_id,
            CourseCompletionTermsAuditEventType::Superseded,
            None,
            None,
        ),
    )
    .await
    .map(|_| ())
}
