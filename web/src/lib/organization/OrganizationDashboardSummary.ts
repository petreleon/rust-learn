import { type OrganizationDashboardAlert } from "./OrganizationDashboardAlert";
import { type OrganizationDashboardCourseSummary } from "./OrganizationDashboardCourseSummary";
import { type OrganizationDashboardMemberSummary } from "./OrganizationDashboardMemberSummary";
import { type OrganizationDashboardOperatorPermissions } from "./OrganizationDashboardOperatorPermissions";
import { type OrganizationDashboardRewardSummary } from "./OrganizationDashboardRewardSummary";
import { type OrganizationDashboardTeacherApplicationSummary } from "./OrganizationDashboardTeacherApplicationSummary";
import { type OrganizationDashboardWalletSummary } from "./OrganizationDashboardWalletSummary";

export type OrganizationDashboardSummary = {
  alerts: OrganizationDashboardAlert[];
  courses: OrganizationDashboardCourseSummary;
  health: {
    alert_count: number;
    status: string;
  };
  members: OrganizationDashboardMemberSummary;
  operator_permissions: OrganizationDashboardOperatorPermissions;
  organization: {
    id: number;
    name: string;
  };
  rewards: OrganizationDashboardRewardSummary;
  teacher_applications: OrganizationDashboardTeacherApplicationSummary;
  wallet: OrganizationDashboardWalletSummary;
};
