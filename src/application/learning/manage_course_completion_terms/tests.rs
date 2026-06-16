mod support;

use bigdecimal::BigDecimal;

use super::*;
use crate::domain::access_control::permissions::Permissions;
use crate::domain::learning::course::CourseCompletionTermsStatus;
use support::{terms_output, FakeStore};

#[tokio::test]
async fn submit_creates_teacher_owned_course_terms() {
    let mut store = FakeStore {
        allowed: vec![Permissions::MANAGE_COURSE_SETTINGS.to_string()],
        ..Default::default()
    };

    let output = submit_course_completion_terms(&mut store, proposal_command())
        .await
        .unwrap();

    assert_eq!(output.teacher_user_id, 11);
    assert_eq!(store.created.unwrap().organization_id, Some(7));
}

#[tokio::test]
async fn submit_rejects_negative_completion_reward() {
    let mut command = proposal_command();
    command.completion_reward_amount = BigDecimal::from(-1);

    let error = submit_course_completion_terms(&mut FakeStore::default(), command)
        .await
        .unwrap_err();

    assert!(matches!(error, CourseCompletionTermsError::InvalidInput(_)));
}

#[tokio::test]
async fn counter_keeps_original_teacher() {
    let mut store = FakeStore {
        allowed: vec![Permissions::SET_REWARD_POLICY.to_string()],
        terms: Some(terms_output(22, CourseCompletionTermsStatus::Submitted)),
        ..Default::default()
    };

    counter_course_completion_terms(
        &mut store,
        CourseCompletionTermsCounterCommand {
            actor_user_id: 99,
            course_id: 5,
            terms_id: 22,
            completion_reward_amount: BigDecimal::from(12),
            max_enrolled_students: 20,
            note: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(store.created.unwrap().teacher_user_id, 11);
}

fn proposal_command() -> CourseCompletionTermsProposalCommand {
    CourseCompletionTermsProposalCommand {
        actor_user_id: 11,
        course_id: 5,
        completion_reward_amount: BigDecimal::from(10),
        max_enrolled_students: 25,
        note: None,
    }
}
