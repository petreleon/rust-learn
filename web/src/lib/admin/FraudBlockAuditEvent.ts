export type FraudBlockAuditEvent = {
  actor_user_id: number | null;
  created_at: string;
  event_type: string;
  fraud_block_id: number;
  id: number;
  reason: string | null;
};
