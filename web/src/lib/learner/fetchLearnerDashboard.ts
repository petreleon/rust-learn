import { DEFAULT_DASHBOARD_COURSE_LIMIT } from "./DEFAULT_DASHBOARD_COURSE_LIMIT";
import { DEFAULT_DASHBOARD_RECOMMENDED_LIMIT } from "./DEFAULT_DASHBOARD_RECOMMENDED_LIMIT";
import { DEFAULT_DASHBOARD_REWARD_LIMIT } from "./DEFAULT_DASHBOARD_REWARD_LIMIT";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { fetchCourseCatalog } from "./fetchCourseCatalog";
import { fetchMyWallet } from "./fetchMyWallet";
import { fetchRewardHistory } from "./fetchRewardHistory";
import { type LearnerDashboardSnapshot } from "./LearnerDashboardSnapshot";
import { type LearnerRequestOptions } from "./LearnerRequestOptions";

export async function fetchLearnerDashboard({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<LearnerDashboardSnapshot> {
  const [enrolledCatalog, recommendedCatalog, rewardHistory, wallet] = await Promise.all([
    fetchCourseCatalog({
      apiRoot,
      enrollmentStatus: "enrolled",
      limit: DEFAULT_DASHBOARD_COURSE_LIMIT,
      timeoutMs,
      token,
    }),
    fetchCourseCatalog({
      apiRoot,
      enrollmentStatus: "available",
      limit: DEFAULT_DASHBOARD_RECOMMENDED_LIMIT,
      timeoutMs,
      token,
    }),
    fetchRewardHistory({
      apiRoot,
      limit: DEFAULT_DASHBOARD_REWARD_LIMIT,
      timeoutMs,
      token,
    }),
    fetchMyWallet({
      apiRoot,
      timeoutMs,
      token,
    }),
  ]);

  return {
    enrolled_catalog: enrolledCatalog,
    recommended_catalog: recommendedCatalog,
    reward_history: rewardHistory,
    wallet,
  };
}
