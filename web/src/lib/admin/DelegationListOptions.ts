import { type AdminRequestOptions } from "./AdminRequestOptions";

export type DelegationListOptions = AdminRequestOptions & {
  active?: boolean | null;
  course_id?: number | null;
  grantee_user_id?: number | null;
  grantor_user_id?: number | null;
  limit?: number;
  offset?: number;
  organization_id?: number | null;
  permission?: string | null;
  scope_type?: string | null;
};
