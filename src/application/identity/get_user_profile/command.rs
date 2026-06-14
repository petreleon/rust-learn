#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetUserProfileCommand {
    pub requester_user_id: i32,
    pub target_user_id: i32,
}
