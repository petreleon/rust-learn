import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type PlatformRewardDashboard } from "./PlatformRewardDashboard";

export async function fetchPlatformRewardDashboard({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions): Promise<PlatformRewardDashboard> {
  return adminJsonRequest({
    apiRoot,
    path: "/reports/platform/reward-dashboard",
    timeoutMs,
    token,
  });
}
