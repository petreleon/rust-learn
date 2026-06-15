use chrono::Utc;
use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};
use serde_json::json;

use super::list_course_reward_candidates;
use crate::application::rewards::list_course_candidates::{
    CourseRewardCandidate, CourseRewardCandidatesError, CourseRewardCandidatesFilter,
    CourseRewardCandidatesQuery,
};
use crate::application::rewards::ports::CourseRewardCandidateStore;
use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[test]
fn non_manager_is_limited_to_own_candidates() {
    let mut store = FakeCourseRewardCandidateStore::student_viewer();

    let candidates = block_on(list_course_reward_candidates(
        &mut store,
        42,
        7,
        CourseRewardCandidatesQuery {
            student_user_id: Some(100),
            status: Some("teacher-approved".to_string()),
            limit: Some(25),
            offset: Some(0),
        },
    ))
    .expect("student viewer should list own candidates");

    assert_eq!(candidates.len(), 1);
    let filter = store.last_filter.as_ref().expect("list filter");
    assert_eq!(filter.student_user_id, Some(42));
    assert_eq!(filter.status.as_deref(), Some("teacher_approved"));
}

#[test]
fn manager_can_keep_requested_student_filter() {
    let mut store = FakeCourseRewardCandidateStore::manager();

    block_on(list_course_reward_candidates(
        &mut store,
        9,
        7,
        CourseRewardCandidatesQuery {
            student_user_id: Some(100),
            status: None,
            limit: None,
            offset: None,
        },
    ))
    .expect("manager should list requested candidates");

    let filter = store.last_filter.as_ref().expect("list filter");
    assert_eq!(filter.student_user_id, Some(100));
}

#[test]
fn rejects_invalid_status_before_listing() {
    let mut store = FakeCourseRewardCandidateStore::manager();

    let error = block_on(list_course_reward_candidates(
        &mut store,
        9,
        7,
        CourseRewardCandidatesQuery {
            student_user_id: None,
            status: Some("not-real".to_string()),
            limit: None,
            offset: None,
        },
    ))
    .expect_err("invalid status should fail");

    assert_eq!(
        error,
        CourseRewardCandidatesError::InvalidStatus(
            "unsupported reward candidate status".to_string()
        )
    );
    assert!(store.last_filter.is_none());
}

#[derive(Clone)]
struct FakeCourseRewardCandidateStore {
    can_approve: bool,
    can_manage_rules: bool,
    can_view_status: bool,
    last_filter: Option<CourseRewardCandidatesFilter>,
}

impl FakeCourseRewardCandidateStore {
    fn student_viewer() -> Self {
        Self {
            can_approve: false,
            can_manage_rules: false,
            can_view_status: true,
            last_filter: None,
        }
    }

    fn manager() -> Self {
        Self {
            can_approve: true,
            can_manage_rules: false,
            can_view_status: true,
            last_filter: None,
        }
    }
}

impl CourseRewardCandidateStore for FakeCourseRewardCandidateStore {
    fn course_exists(
        &mut self,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<(), CourseRewardCandidatesError>> {
        ready(Ok(())).boxed()
    }

    fn can_approve_student_reward_candidate(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>> {
        ready(Ok(self.can_approve)).boxed()
    }

    fn can_manage_course_reward_rules(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>> {
        ready(Ok(self.can_manage_rules)).boxed()
    }

    fn can_view_course_reward_status(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>> {
        ready(Ok(self.can_view_status)).boxed()
    }

    fn list_course_reward_candidates(
        &mut self,
        filter: CourseRewardCandidatesFilter,
    ) -> BoxFuture<'_, Result<Vec<CourseRewardCandidate>, CourseRewardCandidatesError>> {
        self.last_filter = Some(filter.clone());
        ready(Ok(vec![candidate(filter.course_id)])).boxed()
    }
}

fn candidate(course_id: i32) -> CourseRewardCandidate {
    CourseRewardCandidate {
        id: 1,
        course_id,
        student_user_id: 42,
        submitter_user_id: 9,
        source_scope: RewardCandidateSourceScope::Course,
        source_organization_id: None,
        event_type: RewardEventType::CourseCompletion,
        idempotency_key: "course_completion:7:42:manual".to_string(),
        evidence: json!({"completion_percentage": 100}),
        status: RewardCandidateStatus::TeacherApproved,
        teacher_approver_user_id: Some(9),
        teacher_decision_reason: None,
        teacher_decided_at: None,
        amount_reviewer_user_id: None,
        approved_amount: None,
        amount_decision_reason: None,
        amount_decided_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}
