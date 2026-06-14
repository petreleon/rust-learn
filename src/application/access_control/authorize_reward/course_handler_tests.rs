use futures::executor::block_on;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, test_support::FakeRewardAuthorizationStore, RewardAuthorizationAction,
};
use crate::domain::access_control::permission::Permission;

#[test]
fn approve_student_reward_candidate_checks_exact_course_permission() {
    let mut store = FakeRewardAuthorizationStore {
        course_permissions: vec![(42, Permission::ApproveStudentRewardCandidate)],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ApproveStudentRewardCandidate { course_id: 42 },
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.course_checks,
        vec![(42, Permission::ApproveStudentRewardCandidate)]
    );
}

#[test]
fn manage_course_reward_rules_checks_exact_course_permission() {
    let mut store = FakeRewardAuthorizationStore {
        course_permissions: vec![(42, Permission::ManageCourseRewardRules)],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ManageCourseRewardRules { course_id: 42 },
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.course_checks,
        vec![(42, Permission::ManageCourseRewardRules)]
    );
}

#[test]
fn submit_course_reward_event_accepts_submit_or_create_course_permission() {
    let mut store = FakeRewardAuthorizationStore {
        course_permissions: vec![(42, Permission::CreateRewardableCourseEvent)],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::SubmitCourseRewardEvent { course_id: 42 },
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.course_checks,
        vec![
            (42, Permission::SubmitCourseRewardEvent),
            (42, Permission::CreateRewardableCourseEvent)
        ]
    );
}

#[test]
fn submit_organization_course_reward_event_checks_exact_organization_permission() {
    let mut store = FakeRewardAuthorizationStore {
        organization_permissions: vec![(9, Permission::SubmitOrgCourseRewardEvent)],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::SubmitOrganizationCourseRewardEvent { organization_id: 9 },
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.organization_checks,
        vec![(9, Permission::SubmitOrgCourseRewardEvent)]
    );
}

#[test]
fn view_course_reward_status_checks_exact_course_permission() {
    let mut store = FakeRewardAuthorizationStore {
        course_permissions: vec![(42, Permission::ViewCourseRewardStatus)],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ViewCourseRewardStatus { course_id: 42 },
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.course_checks,
        vec![(42, Permission::ViewCourseRewardStatus)]
    );
}
