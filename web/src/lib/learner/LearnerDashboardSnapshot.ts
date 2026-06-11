import { type CourseCatalogResponse } from "./CourseCatalogResponse";
import { type RewardHistoryEntry } from "./RewardHistoryEntry";
import { type WalletSummary } from "./WalletSummary";

export type LearnerDashboardSnapshot = {
  enrolled_catalog: CourseCatalogResponse;
  recommended_catalog: CourseCatalogResponse;
  reward_history: RewardHistoryEntry[];
  wallet: WalletSummary | null;
};
