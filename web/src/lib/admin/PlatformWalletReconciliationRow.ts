export type PlatformWalletReconciliationRow = {
  balance: string;
  external_transaction_count: number;
  internal_transaction_count: number;
  missing_credit_count: number;
  missing_notification_count: number;
  missing_payout_count: number;
  needs_reconciliation_count: number;
  organization_id: number | null;
  owner_type: string;
  reward_record_count: number;
  user_id: number | null;
  wallet_id: number;
};
