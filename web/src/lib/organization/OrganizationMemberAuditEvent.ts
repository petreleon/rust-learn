export type OrganizationMemberAuditEvent = {
  actor_user_id: number | null;
  created_at: string;
  event_type: string;
  id: number;
  organization_id: number;
  reason: string | null;
  role_name: string | null;
  target_user_id: number;
};
