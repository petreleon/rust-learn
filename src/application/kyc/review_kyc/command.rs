#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KycDecisionCommand {
    pub reviewer_user_id: i32,
    pub submission_id: i64,
    pub status: String,
    pub rejection_reason: Option<String>,
}
