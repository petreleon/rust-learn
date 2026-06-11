import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type PlatformRewardCandidatesResponse } from "./PlatformRewardCandidatesResponse";

export async function fetchPlatformRewardCandidates({
  apiRoot = "/api",
  limit,
  offset,
  search,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & {
  limit?: number;
  offset?: number;
  search?: string | null;
  status?: string | null;
}): Promise<PlatformRewardCandidatesResponse> {
  const query = new URLSearchParams();
  const normalizedSearch = search?.trim();
  if (normalizedSearch) {
    query.set("search", normalizedSearch);
  }
  if (status) {
    query.set("status", status);
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }

  const suffix = query.toString();
  return adminJsonRequest({
    apiRoot,
    path: `/reward-candidates/review${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}
