import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type RewardPolicyActivationOptions } from "./RewardPolicyActivationOptions";
import { type RewardPolicyItem } from "./RewardPolicyItem";

export function updateRewardPolicyActivation({
  active,
  apiRoot = "/api",
  policyId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: RewardPolicyActivationOptions): Promise<RewardPolicyItem> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({ active }),
    method: "PUT",
    path: `/reward-policies/${policyId}/activation`,
    timeoutMs,
    token,
  });
}
