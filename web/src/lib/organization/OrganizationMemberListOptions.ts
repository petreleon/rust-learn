import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export type OrganizationMemberListOptions = OrganizationRequestOptions & {
  limit?: number;
  offset?: number;
  organizationId: number;
  permission?: string | null;
  role?: string | null;
  search?: string | null;
};
