import { type PlatformWalletReconciliationRow } from "./PlatformWalletReconciliationRow";

export type PlatformWalletReconciliation = {
  total_external_transactions: number;
  total_internal_transactions: number;
  total_needs_reconciliation: number;
  total_reward_records: number;
  total_wallets: number;
  wallets: PlatformWalletReconciliationRow[];
};
