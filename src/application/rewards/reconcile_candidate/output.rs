use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardReconciliationOutput {
    pub candidate_id: i64,
    pub wallet_credit_created: bool,
    pub notification_created: bool,
    pub external_transaction_link_repaired: bool,
    pub internal_transaction_link_repaired: bool,
    pub final_status: RewardCandidateStatus,
}
