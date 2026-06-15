use chrono::Utc;
use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};
use serde_json::json;

use super::list_reward_candidate_audit;
use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditError, RewardCandidateAuditEvent,
};
use crate::application::rewards::ports::RewardCandidateAuditStore;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[test]
fn lists_events_after_permission_and_candidate_checks() {
    let mut store = FakeRewardCandidateAuditStore::allowed();

    let events = block_on(list_reward_candidate_audit(&mut store, 42, 77))
        .expect("candidate audit should load");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].reward_candidate_id, 77);
    assert!(store.checked_permission);
    assert!(store.checked_candidate);
    assert!(store.listed_events);
}

#[test]
fn rejects_without_listing_candidate_or_events() {
    let mut store = FakeRewardCandidateAuditStore::denied();

    let error = block_on(list_reward_candidate_audit(&mut store, 42, 77))
        .expect_err("candidate audit should be permission gated");

    assert_eq!(
        error,
        RewardCandidateAuditError::PermissionDenied("VIEW_REWARD_AUDIT".to_string())
    );
    assert!(store.checked_permission);
    assert!(!store.checked_candidate);
    assert!(!store.listed_events);
}

struct FakeRewardCandidateAuditStore {
    can_view: bool,
    checked_permission: bool,
    checked_candidate: bool,
    listed_events: bool,
}

impl FakeRewardCandidateAuditStore {
    fn allowed() -> Self {
        Self {
            can_view: true,
            checked_permission: false,
            checked_candidate: false,
            listed_events: false,
        }
    }

    fn denied() -> Self {
        Self {
            can_view: false,
            checked_permission: false,
            checked_candidate: false,
            listed_events: false,
        }
    }
}

impl RewardCandidateAuditStore for FakeRewardCandidateAuditStore {
    fn can_view_reward_audit(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateAuditError>> {
        self.checked_permission = true;
        ready(Ok(self.can_view)).boxed()
    }

    fn reward_candidate_exists(
        &mut self,
        _candidate_id: i64,
    ) -> BoxFuture<'_, Result<(), RewardCandidateAuditError>> {
        self.checked_candidate = true;
        ready(Ok(())).boxed()
    }

    fn list_candidate_audit_events(
        &mut self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardCandidateAuditEvent>, RewardCandidateAuditError>> {
        self.listed_events = true;
        ready(Ok(vec![RewardCandidateAuditEvent {
            id: 1,
            reward_candidate_id: candidate_id,
            actor_user_id: Some(42),
            event_type: "candidate_submitted".to_string(),
            from_status: None,
            to_status: RewardCandidateStatus::PendingTeacherApproval,
            reason: Some("eligible".to_string()),
            metadata: json!({"source": "test"}),
            created_at: Utc::now(),
        }]))
        .boxed()
    }
}
