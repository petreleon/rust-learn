#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginAuthentication {
    pub user_id: i32,
    pub email_verified: bool,
    pub password_hash: Option<String>,
}
