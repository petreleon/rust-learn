import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { humanizeKey, rewardPolicyContext } from "./rewardPolicyDisplay";

export type RewardPolicyInspectionRow = {
  label: string;
  value: string;
};

export function findRewardPolicyById(policies: RewardPolicyItem[], policyId: number | null) {
  return policies.find((policy) => policy.id === policyId) ?? null;
}

export function rewardPolicyInspectionRows(policy: RewardPolicyItem): RewardPolicyInspectionRow[] {
  return [
    { label: "Scope", value: rewardPolicyContext(policy) },
    { label: "Event", value: humanizeKey(policy.event_type) },
    { label: "Amount", value: `${policy.token_amount} tokens` },
    { label: "Multiplier", value: policy.multiplier },
    { label: "Max payout", value: policy.max_payout ? `${policy.max_payout} tokens` : "No cap" },
    { label: "Cooldown", value: `${policy.cooldown_seconds}s` },
    { label: "Payment", value: humanizeKey(policy.payment_strategy) },
    { label: "Version", value: `v${policy.version}` },
    { label: "Activation model", value: policy.active ? "Current active version" : "Historical version" },
    { label: "Organization ID", value: policy.organization_id?.toString() ?? "None" },
    { label: "Course ID", value: policy.course_id?.toString() ?? "None" },
    { label: "Created by", value: policy.created_by_user_id?.toString() ?? "System" },
  ];
}
