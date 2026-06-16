import { type RewardHistoryEntry } from "./RewardHistoryEntry";
import { type WalletDepositIntentAudit } from "./WalletDepositIntentAudit";
import { type WalletSummary } from "./WalletSummary";

export type LearnerWalletSnapshot = {
  reward_history: RewardHistoryEntry[];
  wallet_history: WalletDepositIntentAudit[];
  wallet: WalletSummary | null;
};
