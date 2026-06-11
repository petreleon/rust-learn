import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type PlatformReportSummary } from "./PlatformReportSummary";

export async function fetchPlatformSummary({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions): Promise<PlatformReportSummary> {
  return adminJsonRequest({
    apiRoot,
    path: "/reports/platform/summary",
    timeoutMs,
    token,
  });
}
