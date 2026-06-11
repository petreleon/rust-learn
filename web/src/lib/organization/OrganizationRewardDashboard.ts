import { type OrganizationCourseRewardDashboardRow } from "./OrganizationCourseRewardDashboardRow";
import { type OrganizationWalletBalanceRow } from "./OrganizationWalletBalanceRow";
import { type TeacherApplicationDashboardSummary } from "./TeacherApplicationDashboardSummary";

export type OrganizationRewardDashboard = {
  approved_amount_total: string;
  approved_reward_count: number;
  course_reward_count: number;
  courses: OrganizationCourseRewardDashboardRow[];
  organization_id: number;
  organization_name: string;
  sponsored_teacher_applications: TeacherApplicationDashboardSummary;
  wallet_balance_total: string;
  wallets: OrganizationWalletBalanceRow[];
};
