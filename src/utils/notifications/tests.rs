use super::{
    content_published_notification, enrollment_notification, reward_event_notification,
    reward_wallet_credit_notification, role_assignment_notification,
    teacher_application_notification, worker_failure_notification,
};

#[test]
fn builds_requested_event_notification_messages() {
    assert_eq!(
        enrollment_notification(7, "Rust 101").title,
        "course:enrolled"
    );
    assert_eq!(
        content_published_notification(7, 9, "video").title,
        "content:published"
    );
    assert_eq!(
        role_assignment_notification("course", Some(7), "STUDENT").title,
        "role:assigned"
    );
    assert_eq!(
        worker_failure_notification(10, "object.mp4", 5, "ffmpeg failed").title,
        "worker:job_failed"
    );
    assert_eq!(
        reward_event_notification("42", "token_transfer", Some(99)).title,
        "reward:recorded"
    );
    assert_eq!(
        reward_wallet_credit_notification(7, "Rust 101", "42", 3, 99).title,
        "reward:wallet_credited"
    );
    assert_eq!(
        teacher_application_notification(
            55,
            "approved",
            "approved",
            "course",
            Some("approved by central administration")
        )
        .title,
        "teacher_application:updated"
    );
}
