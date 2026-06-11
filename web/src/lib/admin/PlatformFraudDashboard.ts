import { type FraudBlockDashboardRow } from "./FraudBlockDashboardRow";
import { type FraudBlockScopeSummary } from "./FraudBlockScopeSummary";

export type PlatformFraudDashboard = {
  active_blocks: FraudBlockDashboardRow[];
  active_by_scope: FraudBlockScopeSummary;
  active_total: number;
};
