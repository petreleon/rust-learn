import { type PlatformTeacherApplicationItem } from "./PlatformTeacherApplicationItem";
import { type PlatformTeacherApplicationPermissions } from "./PlatformTeacherApplicationPermissions";
import { type TeacherApplicationDashboardSummary } from "./TeacherApplicationDashboardSummary";
import { type TeacherApplicationStatus } from "./TeacherApplicationStatus";

export type PlatformTeacherApplicationsResponse = {
  applications: PlatformTeacherApplicationItem[];
  limit: number;
  offset: number;
  operator_permissions: PlatformTeacherApplicationPermissions;
  search: string | null;
  status: TeacherApplicationStatus | null;
  summary: TeacherApplicationDashboardSummary;
  total: number;
};
