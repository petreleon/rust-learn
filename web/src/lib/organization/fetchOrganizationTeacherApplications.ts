import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationTeacherApplicationList } from "./OrganizationTeacherApplicationList";
import { type OrganizationTeacherApplicationListOptions } from "./OrganizationTeacherApplicationListOptions";

export async function fetchOrganizationTeacherApplications({
  apiRoot = "/api",
  limit,
  offset,
  organizationId,
  search,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationTeacherApplicationListOptions): Promise<OrganizationTeacherApplicationList> {
  const query = new URLSearchParams();
  const normalizedSearch = search?.trim();
  if (normalizedSearch) {
    query.set("search", normalizedSearch);
  }
  const normalizedStatus = status?.trim();
  if (normalizedStatus) {
    query.set("status", normalizedStatus);
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
    path: `/organizations/${organizationId}/teacher-applications${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}
