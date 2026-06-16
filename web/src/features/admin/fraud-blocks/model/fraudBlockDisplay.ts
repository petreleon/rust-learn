import { type FraudBlockItem } from "@/lib/admin/FraudBlockItem";
import { type FraudBlockStatus } from "@/lib/admin/FraudBlockStatus";

export const fraudBlockScopeOptions = [
  { label: "All scopes", value: "" },
  { label: "Teacher", value: "teacher" },
  { label: "Organization", value: "organization" },
  { label: "Course", value: "course" },
  { label: "Reward policy", value: "reward_policy" },
];

export const fraudBlockCreateScopeOptions = [
  { label: "Select scope...", value: "" },
  { label: "Teacher", value: "teacher" },
  { label: "Organization", value: "organization" },
  { label: "Course", value: "course" },
  { label: "Reward policy", value: "reward_policy" },
];

export const fraudBlockActiveOptions = [
  { label: "Active", value: "true" },
  { label: "Inactive", value: "false" },
  { label: "All", value: "" },
];

export function fraudBlockStatus(block: FraudBlockItem): FraudBlockStatus {
  if (block.revoked_at) return "revoked";
  if (block.expires_at && new Date(block.expires_at) <= new Date()) return "expired";
  return "active";
}

export function fraudBlockTargetLabel(block: FraudBlockItem) {
  if (block.teacher_user_id) return `Teacher user ${block.teacher_user_id}`;
  if (block.organization_id) return `Organization ${block.organization_id}`;
  if (block.course_id) return `Course ${block.course_id}`;
  if (block.reward_policy_id) return `Reward policy ${block.reward_policy_id}`;
  return "Scope target unavailable";
}
