export type RewardPolicyItem = {
  id: number;
  scope_type: string;
  organization_id: number | null;
  course_id: number | null;
  event_type: string;
  version: number;
  token_amount: string;
  multiplier: string;
  max_payout: string | null;
  cooldown_seconds: number;
  payment_strategy: string;
  active: boolean;
  created_by_user_id: number | null;
  created_at: string;
  updated_at: string;
};
