export type OrganizationWalletCompensationRecordAudit = {
  amount: string;
  created_at: string;
  created_by_user_id: number;
  id: number;
  idempotency_key: string;
  internal_transaction_id: number;
  reason: string;
  reward_candidate_id: number;
  transaction_id: number;
  wallet_id: number;
};
