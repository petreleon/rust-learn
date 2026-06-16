import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";

export const rewardPolicyScopeOptions = [
  { label: "All scopes", value: "" },
  { label: "Platform", value: "platform" },
  { label: "Organization", value: "organization" },
  { label: "Course", value: "course" },
];

export const rewardPolicyEventOptions = [
  { label: "All events", value: "" },
  { label: "Course completion", value: "course_completion" },
  { label: "Assessment completion", value: "assessment_completion" },
  { label: "Manual completion", value: "manual_completion" },
  { label: "Administrative adjustment", value: "administrative_adjustment" },
];

export const rewardPolicyCreateEventOptions = rewardPolicyEventOptions.filter((option) => option.value);

export const rewardPolicyActiveOptions = [
  { label: "Active", value: "true" },
  { label: "Inactive", value: "false" },
  { label: "All", value: "" },
];

export const rewardPolicyPaymentOptions = [
  { label: "Treasury transfer", value: "treasury_transfer" },
  { label: "Mint", value: "mint" },
  { label: "Off chain", value: "off_chain" },
];

export function rewardPolicyContext(policy: RewardPolicyItem) {
  if (policy.scope_type === "course") {
    return `Course ${policy.course_id ?? "unknown"}`;
  }
  if (policy.scope_type === "organization") {
    return `Organization ${policy.organization_id ?? "unknown"}`;
  }
  return "Platform";
}

export function humanizeKey(value: string) {
  return value
    .split("_")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}
