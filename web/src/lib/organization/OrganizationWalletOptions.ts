import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export type OrganizationWalletOptions = OrganizationRequestOptions & {
  organizationId: number;
};
