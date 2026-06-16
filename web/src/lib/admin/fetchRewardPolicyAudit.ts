import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type RewardPolicyAuditEvent } from "./RewardPolicyAuditEvent";

export function fetchRewardPolicyAudit({
  apiRoot = "/api",
  policyId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { policyId: number }): Promise<RewardPolicyAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/reward-policies/${policyId}/audit`,
    timeoutMs,
    token,
  });
}
