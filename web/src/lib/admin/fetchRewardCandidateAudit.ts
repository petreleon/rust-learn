import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type RewardAuditEvent } from "./RewardAuditEvent";

export async function fetchRewardCandidateAudit({
  apiRoot = "/api",
  candidateId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { candidateId: number }): Promise<RewardAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/reward-candidates/${candidateId}/audit`,
    timeoutMs,
    token,
  });
}
