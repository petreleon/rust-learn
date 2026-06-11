export type DelegationItem = {
  course_id: number | null;
  created_at: string;
  expires_at: string | null;
  grantee_user_id: number;
  grantor_user_id: number;
  id: number;
  organization_id: number | null;
  permission: string;
  reason: string | null;
  revoked_at: string | null;
  revoked_by_user_id: number | null;
  revoke_reason: string | null;
  scope_type: string;
  updated_at: string;
};
