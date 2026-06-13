use crate::application::learning::course_enrollment::validation::normalize_join_decision;

#[test]
fn normalizes_valid_join_decisions() {
    assert_eq!(normalize_join_decision("approved").unwrap(), "approved");
    assert_eq!(normalize_join_decision("rejected").unwrap(), "rejected");
    assert_eq!(normalize_join_decision("waitlisted").unwrap(), "waitlisted");
    assert_eq!(normalize_join_decision("  APPROVED  ").unwrap(), "approved");
}

#[test]
fn rejects_invalid_join_decision_status() {
    assert!(normalize_join_decision("").is_err());
    assert!(normalize_join_decision("pending").is_err());
    assert!(normalize_join_decision("accepted").is_err());
}
