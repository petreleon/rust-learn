import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type PlatformRewardCandidateItem } from "./PlatformRewardCandidateItem";
import { type RewardCandidateAmountDecisionStatus } from "./RewardCandidateAmountDecisionStatus";

export async function decideRewardAmount({
  apiRoot = "/api",
  candidateId,
  status,
  approvedAmount,
  decisionReason,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & {
  candidateId: number;
  status: RewardCandidateAmountDecisionStatus;
  approvedAmount?: string | null;
  decisionReason?: string | null;
}): Promise<PlatformRewardCandidateItem> {
  const body: Record<string, unknown> = {
    status,
    decision_reason: decisionReason?.trim() || null,
  };
  if (status === "approved" && approvedAmount != null) {
    body.approved_amount = approvedAmount;
  }
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify(body),
    method: "PUT",
    path: `/reward-candidates/${candidateId}/amount-decision`,
    timeoutMs,
    token,
  });
}
