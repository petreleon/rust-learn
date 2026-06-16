import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";

export type RewardPolicyOption = {
  id: number;
  label: string;
};

export function rewardPolicyOption(policy: RewardPolicyItem): RewardPolicyOption {
  const scope = policy.course_id
    ? `Course ${policy.course_id}`
    : policy.organization_id
      ? `Org ${policy.organization_id}`
      : "Platform";
  return {
    id: policy.id,
    label: `${scope} · ${policy.event_type} · ${policy.token_amount} tokens · v${policy.version}`,
  };
}
