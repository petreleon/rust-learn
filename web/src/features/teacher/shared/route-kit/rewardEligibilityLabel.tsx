import { statusLabel } from "./statusLabel";

type RewardEligibility = {
  active_policy_count: number;
  event_types: string[];
  supported: boolean;
};

export function rewardEligibilityLabel(eligibility: RewardEligibility) {
  if (!eligibility.supported) return "Not tracked yet";
  if (!eligibility.active_policy_count) return "No active reward policy";
  const events = eligibility.event_types.map(statusLabel).join(", ");
  return `${eligibility.active_policy_count} active${events ? `: ${events}` : ""}`;
}
