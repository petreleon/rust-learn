import { type OrganizationDashboardSectionGate } from "./OrganizationDashboardSectionGate";

export type OrganizationDashboardMemberSummary = OrganizationDashboardSectionGate & {
  delegated_permission_count: number;
  kyc_ready_count: number;
  total: number;
  verified_email_count: number;
};
