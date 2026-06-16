import { type AdminRequestOptions } from "./AdminRequestOptions";

export type RewardPolicyCreatePayload = {
  active?: boolean | null;
  cooldown_seconds?: number | null;
  course_id?: number | null;
  event_type: string;
  max_payout?: string | null;
  multiplier?: string | null;
  organization_id?: number | null;
  payment_strategy: string;
  scope_type: string;
  token_amount: string;
};

export type RewardPolicyCreateOptions = AdminRequestOptions & {
  payload: RewardPolicyCreatePayload;
};
