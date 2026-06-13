use chrono::Utc;
use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};

use super::list_platform_reward_candidates;
use crate::application::rewards::list_platform_candidates::store::PlatformRewardCandidateStore;
use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidateCourseSummary, PlatformRewardCandidateRecord,
    PlatformRewardCandidateUserSummary, PlatformRewardCandidatesError,
    PlatformRewardCandidatesQuery,
};

#[test]
fn enriches_searches_and_pages_platform_candidates() {
    let mut store = FakePlatformRewardCandidateStore::allowed();

    let output = block_on(list_platform_reward_candidates(
        &mut store,
        9,
        PlatformRewardCandidatesQuery {
            status: Some("teacher-approved".to_string()),
            search: Some("alice".to_string()),
            limit: Some(1),
            offset: Some(0),
        },
    ))
    .expect("platform candidates should list");

    assert_eq!(output.total, 1);
    assert_eq!(output.status.as_deref(), Some("teacher_approved"));
    assert_eq!(output.candidates[0].student.name, "Alice Student");
    assert_eq!(
        output.candidates[0].teacher_approver.as_ref().unwrap().id,
        9
    );
    assert!(output.operator_permissions.can_view_candidates);
    assert!(output.operator_permissions.can_approve_amount);
    assert!(store.counted_candidates);
}

#[test]
fn rejects_without_platform_audit_permission() {
    let mut store = FakePlatformRewardCandidateStore::denied();

    let error = block_on(list_platform_reward_candidates(
        &mut store,
        9,
        PlatformRewardCandidatesQuery::default(),
    ))
    .expect_err("platform candidates should be permission gated");

    assert_eq!(
        error,
        PlatformRewardCandidatesError::PermissionDenied("VIEW_REWARD_AUDIT".to_string())
    );
    assert!(!store.loaded_candidates);
}

struct FakePlatformRewardCandidateStore {
    can_view: bool,
    counted_candidates: bool,
    loaded_candidates: bool,
}

impl FakePlatformRewardCandidateStore {
    fn allowed() -> Self {
        Self {
            can_view: true,
            counted_candidates: false,
            loaded_candidates: false,
        }
    }

    fn denied() -> Self {
        Self {
            can_view: false,
            counted_candidates: false,
            loaded_candidates: false,
        }
    }
}

impl PlatformRewardCandidateStore for FakePlatformRewardCandidateStore {
    fn can_view_reward_audit(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRewardCandidatesError>> {
        ready(Ok(self.can_view)).boxed()
    }

    fn can_approve_reward_amount(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRewardCandidatesError>> {
        ready(Ok(true)).boxed()
    }

    fn list_candidate_records(
        &mut self,
        _status: Option<String>,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardCandidateRecord>, PlatformRewardCandidatesError>>
    {
        self.loaded_candidates = true;
        ready(Ok(vec![candidate_record(1, 42), candidate_record(2, 43)])).boxed()
    }

    fn count_candidate_records(
        &mut self,
        _status: Option<String>,
    ) -> BoxFuture<'_, Result<i64, PlatformRewardCandidatesError>> {
        self.counted_candidates = true;
        ready(Ok(2)).boxed()
    }

    fn load_user_summaries(
        &mut self,
        _user_ids: Vec<i32>,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardCandidateUserSummary>, PlatformRewardCandidatesError>>
    {
        ready(Ok(vec![
            user(42, "Alice Student", "alice@example.com"),
            user(43, "Bob Student", "bob@example.com"),
            user(9, "Teacher", "teacher@example.com"),
        ]))
        .boxed()
    }

    fn load_course_summaries(
        &mut self,
        _course_ids: Vec<i32>,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformRewardCandidateCourseSummary>, PlatformRewardCandidatesError>,
    > {
        ready(Ok(vec![PlatformRewardCandidateCourseSummary {
            id: 7,
            title: "Rust 101".to_string(),
        }]))
        .boxed()
    }
}

fn candidate_record(id: i64, student_user_id: i32) -> PlatformRewardCandidateRecord {
    PlatformRewardCandidateRecord {
        id,
        course_id: 7,
        student_user_id,
        submitter_user_id: 9,
        source_scope: "course".to_string(),
        source_organization_id: None,
        event_type: "course_completion".to_string(),
        status: "teacher_approved".to_string(),
        teacher_approver_user_id: Some(9),
        teacher_decision_reason: Some("well done".to_string()),
        approved_amount: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn user(id: i32, name: &str, email: &str) -> PlatformRewardCandidateUserSummary {
    PlatformRewardCandidateUserSummary {
        id,
        name: name.to_string(),
        email: email.to_string(),
    }
}
