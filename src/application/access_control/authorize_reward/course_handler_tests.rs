use futures::executor::block_on;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, test_support::FakeRewardAuthorizationStore, RewardAuthorizationAction,
};
use crate::domain::access_control::permissions::Permissions;

#[test]
fn approve_student_reward_candidate_checks_exact_course_permission() {
    let mut store = FakeRewardAuthorizationStore {
        course_permissions: vec![(42, Permissions::APPROVE_STUDENT_REWARD_CANDIDATE)],
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
        vec![(42, Permissions::APPROVE_STUDENT_REWARD_CANDIDATE)]
    );
}

#[test]
fn manage_course_reward_rules_checks_exact_course_permission() {
    let mut store = FakeRewardAuthorizationStore {
        course_permissions: vec![(42, Permissions::MANAGE_COURSE_REWARD_RULES)],
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
        vec![(42, Permissions::MANAGE_COURSE_REWARD_RULES)]
    );
}

#[test]
fn submit_course_reward_event_accepts_submit_or_create_course_permission() {
    let mut store = FakeRewardAuthorizationStore {
        course_permissions: vec![(42, Permissions::CREATE_REWARDABLE_COURSE_EVENT)],
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
            (42, Permissions::SUBMIT_COURSE_REWARD_EVENT),
            (42, Permissions::CREATE_REWARDABLE_COURSE_EVENT)
        ]
    );
}

#[test]
fn submit_organization_course_reward_event_checks_exact_organization_permission() {
    let mut store = FakeRewardAuthorizationStore {
        organization_permissions: vec![(9, Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT)],
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
        vec![(9, Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT)]
    );
}

#[test]
fn view_course_reward_status_checks_exact_course_permission() {
    let mut store = FakeRewardAuthorizationStore {
        course_permissions: vec![(42, Permissions::VIEW_COURSE_REWARD_STATUS)],
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
        vec![(42, Permissions::VIEW_COURSE_REWARD_STATUS)]
    );
}
