pub fn teacher_application_notification(
    application_id: i64,
    event_type: impl AsRef<str>,
    status: impl AsRef<str>,
    requested_scope: impl AsRef<str>,
    reason: Option<impl AsRef<str>>,
) -> NotificationMessage {
    let reason_label = reason
        .map(|value| format!(" Reason: {}", compact_text(value, 160)))
        .unwrap_or_default();

    NotificationMessage {
        title: "teacher_application:updated",
        body: format!(
            "Teacher application #{} {} with status {} for {} scope.{}",
            application_id,
            compact_text(event_type, 80),
            compact_text(status, 80),
            compact_text(requested_scope, 80),
            reason_label
        ),
    }
}
