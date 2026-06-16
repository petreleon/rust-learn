use super::terms::{
    CourseCompletionTermsAuditEventType, CourseCompletionTermsStatus, COURSE_TERMS_STATUS_ACTIVE,
};

#[test]
fn exposes_stable_terms_status_keys() {
    assert_eq!(
        CourseCompletionTermsStatus::Active.as_str(),
        COURSE_TERMS_STATUS_ACTIVE
    );
    assert_eq!(CourseCompletionTermsStatus::Countered.as_str(), "countered");
}

#[test]
fn parses_terms_status_and_audit_event() {
    assert_eq!(
        CourseCompletionTermsStatus::parse("submitted").unwrap(),
        CourseCompletionTermsStatus::Submitted
    );
    assert_eq!(
        CourseCompletionTermsAuditEventType::parse("accepted").unwrap(),
        CourseCompletionTermsAuditEventType::Accepted
    );
}

#[test]
fn guards_terminal_terms_transitions() {
    assert!(CourseCompletionTermsStatus::Submitted
        .can_transition_to(CourseCompletionTermsStatus::Countered));
    assert!(CourseCompletionTermsStatus::Accepted
        .can_transition_to(CourseCompletionTermsStatus::Active));
    assert!(!CourseCompletionTermsStatus::Rejected
        .can_transition_to(CourseCompletionTermsStatus::Active));
}
