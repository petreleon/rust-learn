import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export type OrganizationReportOptions = OrganizationRequestOptions & {
  organizationId: number;
};
