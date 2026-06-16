import { type AdminRequestOptions } from "./AdminRequestOptions";

export type RewardPolicyListOptions = AdminRequestOptions & {
  active?: boolean | null;
  course_id?: number | null;
  event_type?: string | null;
  limit?: number;
  offset?: number;
  organization_id?: number | null;
  scope_type?: string | null;
};
