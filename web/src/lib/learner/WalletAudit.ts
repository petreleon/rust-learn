import { type WalletDepositIntentAudit } from "./WalletDepositIntentAudit";
import { type WalletSummary } from "./WalletSummary";

export type WalletAudit = {
  compensation_records: unknown[];
  deposit_intents: WalletDepositIntentAudit[];
  external_transactions: unknown[];
  internal_transactions: unknown[];
  reward_records: unknown[];
  wallet: WalletSummary;
};
