import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export type OrganizationTeacherApplicationListOptions = OrganizationRequestOptions & {
  limit?: number;
  offset?: number;
  organizationId: number;
  search?: string | null;
  status?: string | null;
};
