import { type RewardHistoryEntry } from "./RewardHistoryEntry";
import { type WalletSummary } from "./WalletSummary";

export type LearnerWalletSnapshot = {
  reward_history: RewardHistoryEntry[];
  wallet: WalletSummary | null;
};
