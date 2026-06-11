import { type OrganizationTeacherApplicationItem } from "./OrganizationTeacherApplicationItem";
import { type OrganizationTeacherApplicationOperatorPermissions } from "./OrganizationTeacherApplicationOperatorPermissions";
import { type TeacherApplicationDashboardSummary } from "./TeacherApplicationDashboardSummary";

export type OrganizationTeacherApplicationList = {
  applications: OrganizationTeacherApplicationItem[];
  limit: number;
  offset: number;
  operator_permissions: OrganizationTeacherApplicationOperatorPermissions;
  organization: {
    id: number;
    name: string;
  };
  search: string | null;
  status: string | null;
  summary: TeacherApplicationDashboardSummary;
  total: number;
};
