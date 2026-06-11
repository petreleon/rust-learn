import { type PlatformTeacherApplicationAuditSummary } from "./PlatformTeacherApplicationAuditSummary";
import { type PlatformTeacherApplicationCourse } from "./PlatformTeacherApplicationCourse";
import { type PlatformTeacherApplicationOrganization } from "./PlatformTeacherApplicationOrganization";
import { type PlatformTeacherApplicationUser } from "./PlatformTeacherApplicationUser";
import { type TeacherApplicationScope } from "./TeacherApplicationScope";
import { type TeacherApplicationStatus } from "./TeacherApplicationStatus";

export type PlatformTeacherApplicationItem = {
  applicant: PlatformTeacherApplicationUser;
  audit: PlatformTeacherApplicationAuditSummary;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  experience_summary: string;
  id: number;
  portfolio_links: string[];
  requested_course: PlatformTeacherApplicationCourse | null;
  requested_organization: PlatformTeacherApplicationOrganization | null;
  requested_scope: TeacherApplicationScope;
  reviewer: PlatformTeacherApplicationUser | null;
  sponsor_organization: PlatformTeacherApplicationOrganization | null;
  status: TeacherApplicationStatus;
  updated_at: string;
};
