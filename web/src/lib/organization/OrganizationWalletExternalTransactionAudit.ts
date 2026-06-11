export type OrganizationWalletExternalTransactionAudit = {
  amount: string;
  blockchain_address: string;
  chain_id: number | null;
  contract_address: string | null;
  event_type: string | null;
  external_transaction_id: number;
  from_address: string | null;
  log_index: number | null;
  reward_candidate_id: number | null;
  to_address: string | null;
  transaction_hash: string | null;
  transaction_id: number;
};
