import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type PlatformTeacherApplicationListOptions } from "./PlatformTeacherApplicationListOptions";
import { type PlatformTeacherApplicationsResponse } from "./PlatformTeacherApplicationsResponse";

export async function fetchPlatformTeacherApplications({
  apiRoot = "/api",
  limit,
  offset,
  search,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: PlatformTeacherApplicationListOptions): Promise<PlatformTeacherApplicationsResponse> {
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
    path: `/teacher-applications/review${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}
