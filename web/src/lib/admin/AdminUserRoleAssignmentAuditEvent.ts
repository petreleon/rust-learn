export type AdminUserRoleAssignmentAuditEvent = {
  actor_user_id: number | null;
  created_at: string;
  event_type: string;
  id: number;
  platform_role_id: number | null;
  role_name: string;
  target_user_id: number;
};
