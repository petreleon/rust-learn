import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type FraudBlockCreateOptions } from "./FraudBlockCreateOptions";
import { type FraudBlockItem } from "./FraudBlockItem";

export async function createFraudBlock({
  apiRoot = "/api",
  course_id,
  evidence_reference,
  expires_at,
  organization_id,
  reason,
  reward_policy_id,
  scope_type,
  teacher_user_id,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FraudBlockCreateOptions): Promise<FraudBlockItem> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({
      course_id: course_id ?? null,
      evidence_reference: evidence_reference?.trim() || null,
      expires_at: expires_at ?? null,
      organization_id: organization_id ?? null,
      reason: reason.trim(),
      reward_policy_id: reward_policy_id ?? null,
      scope_type,
      teacher_user_id: teacher_user_id ?? null,
    }),
    method: "POST",
    path: "/reward-fraud-blocks",
    timeoutMs,
    token,
  });
}
