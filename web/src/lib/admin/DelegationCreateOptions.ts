import { type AdminRequestOptions } from "./AdminRequestOptions";

export type DelegationCreateOptions = AdminRequestOptions & {
  course_id?: number | null;
  expires_at?: string | null;
  grantee_user_id: number;
  organization_id?: number | null;
  permission: string;
  reason?: string | null;
  scope_type: string;
};
