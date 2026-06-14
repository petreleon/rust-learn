#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationNotificationOutcome {
    pub recipient_count: usize,
    pub sent_count: usize,
    pub failed_count: usize,
}
