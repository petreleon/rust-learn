use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditError, RewardCandidateAuditEvent,
};
use crate::infra::postgres::models::reward_audit_event::RewardAuditEvent;
use crate::infra::postgres::rewards::reward_vocabulary::{
    parse_audit_event_type, parse_candidate_status,
};

pub(super) fn map_reward_candidate_audit_event(
    event: RewardAuditEvent,
) -> Result<RewardCandidateAuditEvent, RewardCandidateAuditError> {
    let from_status = event
        .from_status
        .as_deref()
        .map(|status| {
            parse_candidate_status(status, |message| {
                RewardCandidateAuditError::InvalidStatus(message)
            })
        })
        .transpose()?;
    let to_status = parse_candidate_status(&event.to_status, |message| {
        RewardCandidateAuditError::InvalidStatus(message)
    })?;
    let event_type = parse_audit_event_type(&event.event_type, |message| {
        RewardCandidateAuditError::Database(message)
    })?;

    Ok(RewardCandidateAuditEvent {
        id: event.id,
        reward_candidate_id: event.reward_candidate_id,
        actor_user_id: event.actor_user_id,
        event_type,
        from_status,
        to_status,
        reason: event.reason,
        metadata: event.metadata,
        created_at: event.created_at,
    })
}

pub(super) fn map_reward_candidate_audit_error(
    error: diesel::result::Error,
) -> RewardCandidateAuditError {
    match error {
        diesel::result::Error::NotFound => RewardCandidateAuditError::NotFound,
        other => RewardCandidateAuditError::Database(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::json;

    use super::map_reward_candidate_audit_event;
    use crate::application::rewards::list_candidate_audit::RewardCandidateAuditError;
    use crate::domain::rewards::audit::RewardAuditEventType;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;
    use crate::infra::postgres::models::reward_audit_event::RewardAuditEvent;

    #[test]
    fn maps_known_candidate_statuses_into_domain_statuses() {
        let mapped = map_reward_candidate_audit_event(event(
            Some("pending_teacher_approval"),
            "teacher_approved",
        ))
        .expect("known statuses should map");

        assert_eq!(
            mapped.from_status,
            Some(RewardCandidateStatus::PendingTeacherApproval)
        );
        assert_eq!(mapped.to_status, RewardCandidateStatus::TeacherApproved);
        assert_eq!(mapped.event_type, RewardAuditEventType::TeacherDecision);
    }

    #[test]
    fn rejects_unknown_to_status_at_infra_boundary() {
        assert_eq!(
            map_reward_candidate_audit_event(event(None, "not_real")).unwrap_err(),
            RewardCandidateAuditError::InvalidStatus(
                "unknown reward candidate status 'not_real'".to_string()
            )
        );
    }

    #[test]
    fn rejects_unknown_from_status_at_infra_boundary() {
        assert_eq!(
            map_reward_candidate_audit_event(event(Some("not_real"), "teacher_approved"))
                .unwrap_err(),
            RewardCandidateAuditError::InvalidStatus(
                "unknown reward candidate status 'not_real'".to_string()
            )
        );
    }

    fn event(from_status: Option<&str>, to_status: &str) -> RewardAuditEvent {
        RewardAuditEvent {
            id: 1,
            reward_candidate_id: 2,
            actor_user_id: Some(3),
            event_type: "teacher_decision".to_string(),
            from_status: from_status.map(str::to_string),
            to_status: to_status.to_string(),
            reason: Some("ok".to_string()),
            metadata: json!({}),
            created_at: Utc::now(),
        }
    }
}
