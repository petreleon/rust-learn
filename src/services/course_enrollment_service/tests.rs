use super::*;
use crate::models::course_join_request::{
    COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_REJECTED, COURSE_JOIN_STATUS_WAITLISTED,
};

#[test]
fn normalizes_valid_decisions() {
    assert_eq!(
        normalize_join_decision(COURSE_JOIN_STATUS_APPROVED).unwrap(),
        "approved"
    );
    assert_eq!(
        normalize_join_decision(COURSE_JOIN_STATUS_REJECTED).unwrap(),
        "rejected"
    );
    assert_eq!(
        normalize_join_decision(COURSE_JOIN_STATUS_WAITLISTED).unwrap(),
        "waitlisted"
    );
    assert_eq!(normalize_join_decision("  APPROVED  ").unwrap(), "approved");
}

#[test]
fn rejects_invalid_decision() {
    assert!(normalize_join_decision("").is_err());
    assert!(normalize_join_decision("pending").is_err());
    assert!(normalize_join_decision("accepted").is_err());
}
