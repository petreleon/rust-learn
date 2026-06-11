import { type OrganizationDashboardSectionGate } from "./OrganizationDashboardSectionGate";

export type OrganizationDashboardWalletSummary = OrganizationDashboardSectionGate & {
  balance_total: string;
  wallet_count: number;
};
