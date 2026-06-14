#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordResetRecipient {
    pub user_id: i32,
    pub email: String,
    pub name: String,
}
