#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationEmailTarget {
    pub user_id: i32,
    pub email: String,
    pub name: String,
    pub email_verified: bool,
}
