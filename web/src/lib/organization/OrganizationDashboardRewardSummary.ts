import { type OrganizationDashboardSectionGate } from "./OrganizationDashboardSectionGate";

export type OrganizationDashboardRewardSummary = OrganizationDashboardSectionGate & {
  approved_amount_total: string;
  approved_reward_count: number;
  failed_count: number;
  needs_reconciliation_count: number;
  reward_candidate_count: number;
};
