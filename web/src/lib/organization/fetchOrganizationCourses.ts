import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationCourseList } from "./OrganizationCourseList";
import { type OrganizationCourseListOptions } from "./OrganizationCourseListOptions";

export async function fetchOrganizationCourses({
  apiRoot = "/api",
  lifecycleStatus,
  limit,
  offset,
  organizationId,
  rewardAvailable,
  search,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationCourseListOptions): Promise<OrganizationCourseList> {
  const query = new URLSearchParams();
  const normalizedSearch = search?.trim();
  if (normalizedSearch) {
    query.set("search", normalizedSearch);
  }
  const normalizedLifecycleStatus = lifecycleStatus?.trim();
  if (normalizedLifecycleStatus) {
    query.set("lifecycle_status", normalizedLifecycleStatus);
  }
  if (typeof rewardAvailable === "boolean") {
    query.set("reward_available", String(rewardAvailable));
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }

  const suffix = query.toString();
  return organizationJsonRequest({
    apiRoot,
    path: `/organizations/${organizationId}/courses${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}
