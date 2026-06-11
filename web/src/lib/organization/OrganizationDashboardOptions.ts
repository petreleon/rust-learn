import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export type OrganizationDashboardOptions = OrganizationRequestOptions & {
  organizationId: number;
};
