import { adminJsonRequest } from "./adminJsonRequest";
import { type RewardPolicyCreateOptions } from "./RewardPolicyCreateOptions";
import { type RewardPolicyItem } from "./RewardPolicyItem";

export function createRewardPolicy({
  apiRoot = "/api",
  payload,
  timeoutMs,
  token,
}: RewardPolicyCreateOptions): Promise<RewardPolicyItem> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify(payload),
    method: "POST",
    path: "/reward-policies",
    timeoutMs,
    token,
  });
}
