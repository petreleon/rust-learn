#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveNotificationPreferencesCommand {
    pub user_id: i32,
    pub email_enabled: bool,
    pub push_enabled: bool,
}
