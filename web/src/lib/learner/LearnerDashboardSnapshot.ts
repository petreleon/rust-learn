import { type CourseCatalogResponse } from "./CourseCatalogResponse";
import { type CourseProgress } from "./CourseProgress";
import { type RewardHistoryEntry } from "./RewardHistoryEntry";
import { type WalletSummary } from "./WalletSummary";

export type LearnerDashboardSnapshot = {
  enrolled_catalog: CourseCatalogResponse;
  progress_by_course: Record<number, CourseProgress>;
  recommended_catalog: CourseCatalogResponse;
  reward_history: RewardHistoryEntry[];
  wallet: WalletSummary | null;
};
