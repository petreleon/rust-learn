import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationMemberList } from "./OrganizationMemberList";
import { type OrganizationMemberListOptions } from "./OrganizationMemberListOptions";

export async function fetchOrganizationMembers({
  apiRoot = "/api",
  limit,
  offset,
  organizationId,
  permission,
  role,
  search,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationMemberListOptions): Promise<OrganizationMemberList> {
  const query = new URLSearchParams();
  const normalizedSearch = search?.trim();
  if (normalizedSearch) {
    query.set("search", normalizedSearch);
  }
  const normalizedRole = role?.trim();
  if (normalizedRole) {
    query.set("role", normalizedRole);
  }
  const normalizedPermission = permission?.trim();
  if (normalizedPermission) {
    query.set("permission", normalizedPermission);
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
    path: `/organizations/${organizationId}/members${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}
