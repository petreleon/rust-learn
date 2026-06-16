import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { type TeacherRewardCandidate } from "@/lib/teacher/TeacherRewardCandidate";
import { statusLabel } from "@/features/teacher/shared/route-kit/statusLabel";

export function rewardPolicyCoverageSummary(course: TeacherCourseDashboardItem): string {
  const { rewards } = course;
  if (!rewards.available || !rewards.active_policy_count) {
    return "No active reward policy covers this course yet. New reward evidence will fail until a policy is active.";
  }

  const events = rewards.event_types.map(statusLabel).join(", ");
  const amounts = rewards.token_amounts.join(", ");
  const strategies = rewards.payment_strategies.map(statusLabel).join(", ");
  const count = rewards.active_policy_count;
  const amountText = amounts ? ` for ${amounts} tokens` : "";
  const strategyText = strategies ? ` via ${strategies}` : "";

  return `${count} active ${policyNoun(count)} ${coverVerb(count)} ${events || "course reward events"}${amountText}${strategyText}.`;
}

export function rewardCandidatePolicyExplanation(
  candidate: TeacherRewardCandidate,
  course: TeacherCourseDashboardItem,
): string {
  const event = statusLabel(candidate.event_type);
  const { rewards } = course;
  if (!rewards.available || !rewards.active_policy_count) {
    return `No active reward policy currently covers ${event}.`;
  }
  if (!rewards.event_types.includes(candidate.event_type)) {
    return `Active policies exist, but none currently cover ${event}.`;
  }
  const amounts = rewards.token_amounts.join(", ");
  const amountText = amounts ? ` Active course policy amounts: ${amounts} tokens.` : "";
  return `Eligible through an active ${event} policy.${amountText}`;
}

function policyNoun(count: number): string {
  return count === 1 ? "policy" : "policies";
}

function coverVerb(count: number): string {
  return count === 1 ? "covers" : "cover";
}
