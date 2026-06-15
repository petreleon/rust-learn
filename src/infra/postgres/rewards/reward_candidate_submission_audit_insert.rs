use diesel_async::{AsyncConnection, AsyncPgConnection};
use serde_json::json;

use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
};
use crate::domain::rewards::audit::RewardAuditEventType;
use crate::infra::postgres::models::reward_audit_event::NewRewardAuditEvent;
use crate::infra::postgres::models::reward_candidate::NewRewardCandidate;
use crate::infra::postgres::rewards::reward_audit_records::create_reward_audit_event;
use crate::infra::postgres::rewards::reward_candidate_records::create_candidate;
use crate::infra::postgres::rewards::reward_candidate_submission_mappers::{
    map_reward_candidate_submission, map_reward_candidate_submission_error,
};

pub(super) async fn create_candidate_with_audit(
    conn: &mut AsyncPgConnection,
    new_candidate: NewRewardCandidate,
    actor_user_id: i32,
) -> Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError> {
    let created = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let created = create_candidate(conn, new_candidate).await?;
                create_reward_audit_event(
                    conn,
                    NewRewardAuditEvent {
                        reward_candidate_id: created.id,
                        actor_user_id: Some(actor_user_id),
                        event_type: RewardAuditEventType::CandidateSubmitted
                            .as_str()
                            .to_string(),
                        from_status: None,
                        to_status: created.status.clone(),
                        reason: None,
                        metadata: json!({
                            "course_id": created.course_id,
                            "student_user_id": created.student_user_id,
                            "source_scope": created.source_scope.clone(),
                            "source_organization_id": created.source_organization_id,
                            "event_type": created.event_type.clone(),
                            "idempotency_key": created.idempotency_key.clone(),
                        }),
                    },
                )
                .await?;

                Ok(created)
            })
        })
        .await
        .map_err(map_reward_candidate_submission_error)?;

    map_reward_candidate_submission(created)
}
