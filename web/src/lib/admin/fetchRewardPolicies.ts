import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type RewardPolicyItem } from "./RewardPolicyItem";
import { type RewardPolicyListOptions } from "./RewardPolicyListOptions";

export async function fetchRewardPolicies({
  active,
  apiRoot = "/api",
  course_id,
  event_type,
  limit,
  offset,
  organization_id,
  scope_type,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: RewardPolicyListOptions): Promise<RewardPolicyItem[]> {
  const query = new URLSearchParams();
  if (typeof active === "boolean") query.set("active", String(active));
  if (typeof course_id === "number") query.set("course_id", String(course_id));
  if (event_type) query.set("event_type", event_type);
  if (typeof limit === "number") query.set("limit", String(limit));
  if (typeof offset === "number") query.set("offset", String(offset));
  if (typeof organization_id === "number") query.set("organization_id", String(organization_id));
  if (scope_type) query.set("scope_type", scope_type);
  const suffix = query.toString();

  return adminJsonRequest({
    apiRoot,
    path: `/reward-policies${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}
