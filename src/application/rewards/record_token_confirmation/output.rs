#[derive(Debug, Clone, PartialEq)]
pub struct RewardTokenConfirmationOutput {
    pub candidate_id: i64,
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub payout_record_id: i64,
    pub inserted_external_transaction: bool,
}
