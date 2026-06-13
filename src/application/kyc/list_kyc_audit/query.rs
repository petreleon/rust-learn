#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KycAuditQuery {
    pub reviewer_user_id: i32,
    pub submission_id: i64,
}
