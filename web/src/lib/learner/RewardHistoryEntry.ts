export type RewardHistoryEntry = {
  approved_amount: string | null;
  course_id: number;
  course_title: string;
  created_at: string;
  event_type: string;
  reward_candidate_id: number;
  status: string;
  token_transaction: {
    amount: string;
    blockchain_address: string;
    chain_id: number | null;
    external_transaction_id: number;
    payout_transaction_id: number;
    recorded_at: string;
    transaction_hash: string | null;
  } | null;
  updated_at: string;
  wallet_credit: {
    amount: string;
    credited_at: string;
    internal_transaction_id: number;
    reward_wallet_credit_record_id: number;
    transaction_id: number;
    wallet_id: number;
  } | null;
};
