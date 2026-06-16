export type RewardPolicyAuditEvent = {
  actor_user_id: number | null;
  created_at: string;
  event_type: string;
  id: number;
  new_active: boolean;
  previous_active: boolean | null;
  reward_policy_id: number;
};
