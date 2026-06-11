import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type PlatformFraudDashboard } from "./PlatformFraudDashboard";

export async function fetchPlatformFraudDashboard({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions): Promise<PlatformFraudDashboard> {
  return adminJsonRequest({
    apiRoot,
    path: "/reports/platform/fraud-dashboard",
    timeoutMs,
    token,
  });
}
