use super::*;
use crate::models::teacher_application::{
    NewTeacherApplication, TeacherApplication, TEACHER_APPLICATION_SCOPE_COURSE,
    TEACHER_APPLICATION_SCOPE_ORGANIZATION, TEACHER_APPLICATION_SCOPE_PLATFORM,
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use chrono::Utc;
use serde_json::json;

// ── normalize_scope ──

#[test]
fn normalizes_scope_variants() {
    assert_eq!(
        normalize_scope(TEACHER_APPLICATION_SCOPE_PLATFORM).unwrap(),
        "platform"
    );
    assert_eq!(
        normalize_scope(TEACHER_APPLICATION_SCOPE_ORGANIZATION).unwrap(),
        "organization"
    );
    assert_eq!(
        normalize_scope(TEACHER_APPLICATION_SCOPE_COURSE).unwrap(),
        "course"
    );
    assert_eq!(normalize_scope("  COURSE  ").unwrap(), "course");
    assert_eq!(normalize_scope("Platform").unwrap(), "platform");
}

#[test]
fn rejects_invalid_scope() {
    assert!(normalize_scope("").is_err());
    assert!(normalize_scope("unknown").is_err());
    assert!(normalize_scope("school").is_err());
}

// ── normalize_status ──

#[test]
fn normalizes_status_variants() {
    assert_eq!(
        normalize_status(TEACHER_APPLICATION_STATUS_SUBMITTED).unwrap(),
        "submitted"
    );
    assert_eq!(
        normalize_status(TEACHER_APPLICATION_STATUS_NEEDS_CHANGES).unwrap(),
        "needs_changes"
    );
    assert_eq!(
        normalize_status(TEACHER_APPLICATION_STATUS_APPROVED).unwrap(),
        "approved"
    );
    assert_eq!(
        normalize_status(TEACHER_APPLICATION_STATUS_REJECTED).unwrap(),
        "rejected"
    );
    assert_eq!(normalize_status("  APPROVED  ").unwrap(), "approved");
}

#[test]
fn rejects_invalid_status() {
    assert!(normalize_status("").is_err());
    assert!(normalize_status("pending").is_err());
    assert!(normalize_status("in_review").is_err());
}

// ── normalize_optional_text ──

#[test]
fn returns_trimmed() {
    assert_eq!(
        normalize_optional_text(Some("  text  ".into())),
        Some("text".into())
    );
}

#[test]
fn returns_none_for_empty() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("".into())), None);
    assert_eq!(normalize_optional_text(Some("   ".into())), None);
}
