import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export type OrganizationCourseListOptions = OrganizationRequestOptions & {
  lifecycleStatus?: string | null;
  limit?: number;
  offset?: number;
  organizationId: number;
  rewardAvailable?: boolean | null;
  search?: string | null;
};
