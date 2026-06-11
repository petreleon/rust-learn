export type RewardAuditEvent = {
  actor_user_id: number | null;
  created_at: string;
  event_type: string;
  from_status: string | null;
  id: number;
  metadata: Record<string, unknown>;
  reason: string | null;
  reward_candidate_id: number;
  to_status: string;
};
