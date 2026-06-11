import { type RewardCandidateDashboardRow } from "./RewardCandidateDashboardRow";
import { type RewardCandidateDashboardSummary } from "./RewardCandidateDashboardSummary";
import { type RewardExecutionFailureRow } from "./RewardExecutionFailureRow";
import { type RewardReconciliationMismatchRow } from "./RewardReconciliationMismatchRow";
import { type TeacherApplicationDashboardSummary } from "./TeacherApplicationDashboardSummary";

export type PlatformRewardDashboard = {
  payout_failure_count: number;
  payout_failures: RewardExecutionFailureRow[];
  pending_amount_approval_count: number;
  pending_amount_approvals: RewardCandidateDashboardRow[];
  reconciliation_mismatch_count: number;
  reconciliation_mismatches: RewardReconciliationMismatchRow[];
  reward_candidates: RewardCandidateDashboardSummary;
  teacher_applications: TeacherApplicationDashboardSummary;
};
