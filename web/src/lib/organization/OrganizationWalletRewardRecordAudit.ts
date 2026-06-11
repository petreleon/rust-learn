export type OrganizationWalletRewardRecordAudit = {
  approved_amount: string | null;
  candidate_status: string;
  created_at: string;
  external_transaction_id: number | null;
  internal_transaction_id: number | null;
  notification_id: number | null;
  notified_at: string | null;
  payout_record_id: number | null;
  payout_transaction_id: number | null;
  reconciliation_status: string;
  reward_candidate_id: number;
  updated_at: string;
  wallet_credit_record_id: number | null;
  wallet_credit_transaction_id: number | null;
};
