export type FraudBlockItem = {
  course_id: number | null;
  created_at: string;
  created_by_user_id: number;
  evidence_reference: string | null;
  expires_at: string | null;
  id: number;
  organization_id: number | null;
  reason: string;
  reward_policy_id: number | null;
  revoked_at: string | null;
  revoked_by_user_id: number | null;
  scope_type: string;
  teacher_user_id: number | null;
  updated_at: string;
};
