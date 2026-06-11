import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { systemJsonRequest } from "./systemJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type PlatformSystemStatus } from "./PlatformSystemStatus";
import { type SystemLiveness } from "./SystemLiveness";
import { type SystemReadiness } from "./SystemReadiness";

export async function fetchPlatformSystemStatus({
  apiRoot = "",
  timeoutMs = DEFAULT_TIMEOUT_MS,
}: Omit<AdminRequestOptions, "token"> = {}): Promise<PlatformSystemStatus> {
  const [liveness, readiness] = await Promise.all([
    systemJsonRequest<SystemLiveness>({ apiRoot, path: "/health", timeoutMs }),
    systemJsonRequest<SystemReadiness>({
      acceptedStatuses: [503],
      apiRoot,
      path: "/ready",
      timeoutMs,
    }),
  ]);

  return {
    liveness,
    readiness,
  };
}
