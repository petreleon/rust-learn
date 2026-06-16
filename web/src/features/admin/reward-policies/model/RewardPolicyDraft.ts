import { type RewardPolicyCreatePayload } from "@/lib/admin/RewardPolicyCreateOptions";

export type RewardPolicyDraft = {
  active: boolean;
  cooldownSeconds: string;
  courseId: string;
  eventType: string;
  maxPayout: string;
  multiplier: string;
  organizationId: string;
  paymentStrategy: string;
  scopeType: string;
  tokenAmount: string;
};

export const defaultRewardPolicyDraft: RewardPolicyDraft = {
  active: true,
  cooldownSeconds: "0",
  courseId: "",
  eventType: "course_completion",
  maxPayout: "",
  multiplier: "1",
  organizationId: "",
  paymentStrategy: "treasury_transfer",
  scopeType: "platform",
  tokenAmount: "",
};

export function rewardPolicyPayload(draft: RewardPolicyDraft): RewardPolicyCreatePayload {
  return {
    active: draft.active,
    cooldown_seconds: optionalNumber(draft.cooldownSeconds) ?? 0,
    course_id: draft.scopeType === "course" ? requiredNumber(draft.courseId) : null,
    event_type: draft.eventType,
    max_payout: optionalText(draft.maxPayout),
    multiplier: optionalText(draft.multiplier) ?? "1",
    organization_id: scopedOrganizationId(draft),
    payment_strategy: draft.paymentStrategy,
    scope_type: draft.scopeType,
    token_amount: draft.tokenAmount.trim(),
  };
}

export function validateRewardPolicyDraft(draft: RewardPolicyDraft): string | null {
  if (!draft.tokenAmount.trim()) return "Token amount is required.";
  if (draft.scopeType === "organization" && !draft.organizationId.trim()) {
    return "Organization policies require an organization id.";
  }
  if (draft.scopeType === "course" && !draft.courseId.trim()) {
    return "Course policies require a course id.";
  }
  if (numberInvalid(draft.organizationId) || numberInvalid(draft.courseId)) {
    return "Scope ids must be positive whole numbers.";
  }
  if (numberInvalid(draft.cooldownSeconds, true)) {
    return "Cooldown seconds must be a non-negative whole number.";
  }
  return null;
}

function scopedOrganizationId(draft: RewardPolicyDraft) {
  if (draft.scopeType === "platform") return null;
  return optionalNumber(draft.organizationId);
}

function optionalText(value: string) {
  const trimmed = value.trim();
  return trimmed ? trimmed : null;
}

function optionalNumber(value: string) {
  const trimmed = value.trim();
  return trimmed ? requiredNumber(trimmed) : null;
}

function requiredNumber(value: string) {
  return Number.parseInt(value.trim(), 10);
}

function numberInvalid(value: string, allowZero = false) {
  const trimmed = value.trim();
  if (!trimmed) return false;
  const parsed = Number.parseInt(trimmed, 10);
  return !/^\d+$/.test(trimmed) || (allowZero ? parsed < 0 : parsed <= 0);
}
