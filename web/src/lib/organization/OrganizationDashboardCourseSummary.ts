import { type OrganizationDashboardSectionGate } from "./OrganizationDashboardSectionGate";

export type OrganizationDashboardCourseSummary = OrganizationDashboardSectionGate & {
  approved: number;
  archived: number;
  draft: number;
  needs_changes: number;
  published: number;
  submitted: number;
  suspended: number;
  total: number;
};
