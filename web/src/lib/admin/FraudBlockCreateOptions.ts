import { type AdminRequestOptions } from "./AdminRequestOptions";

export type FraudBlockCreateOptions = AdminRequestOptions & {
  scope_type: string;
  teacher_user_id?: number | null;
  organization_id?: number | null;
  course_id?: number | null;
  reward_policy_id?: number | null;
  reason: string;
  evidence_reference?: string | null;
  expires_at?: string | null;
};
