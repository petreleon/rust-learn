import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type DelegationItem } from "./DelegationItem";
import { type DelegationListOptions } from "./DelegationListOptions";

type DelegationListResponse = {
  delegations: DelegationItem[];
  limit: number;
  offset: number;
  total: number;
};

export async function fetchDelegations({
  apiRoot = "/api",
  active,
  course_id,
  grantee_user_id,
  grantor_user_id,
  limit,
  offset,
  organization_id,
  permission,
  scope_type,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: DelegationListOptions): Promise<DelegationListResponse> {
  const query = new URLSearchParams();
  if (typeof active === "boolean") {
    query.set("active", String(active));
  }
  if (typeof course_id === "number") {
    query.set("course_id", String(course_id));
  }
  if (typeof grantee_user_id === "number") {
    query.set("grantee_user_id", String(grantee_user_id));
  }
  if (typeof grantor_user_id === "number") {
    query.set("grantor_user_id", String(grantor_user_id));
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  if (typeof organization_id === "number") {
    query.set("organization_id", String(organization_id));
  }
  if (permission) {
    query.set("permission", permission);
  }
  if (scope_type) {
    query.set("scope_type", scope_type);
  }
  const suffix = query.toString();
  const response = await adminJsonRequest<DelegationItem[] | DelegationListResponse>({
    apiRoot,
    path: `/delegated-permissions${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
  if (Array.isArray(response)) {
    return {
      delegations: response,
      limit: limit ?? response.length,
      offset: offset ?? 0,
      total: response.length,
    };
  }
  return response;
}
