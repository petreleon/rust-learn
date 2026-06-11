import { type OrganizationTeacherApplicationAuditSummary } from "./OrganizationTeacherApplicationAuditSummary";
import { type OrganizationTeacherApplicationCourse } from "./OrganizationTeacherApplicationCourse";
import { type OrganizationTeacherApplicationOrganization } from "./OrganizationTeacherApplicationOrganization";
import { type OrganizationTeacherApplicationUser } from "./OrganizationTeacherApplicationUser";

export type OrganizationTeacherApplicationItem = {
  applicant: OrganizationTeacherApplicationUser;
  audit: OrganizationTeacherApplicationAuditSummary;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  experience_summary: string;
  id: number;
  portfolio_links: string[];
  requested_course: OrganizationTeacherApplicationCourse | null;
  requested_for_this_organization: boolean;
  requested_organization: OrganizationTeacherApplicationOrganization | null;
  requested_scope: string;
  reviewer: OrganizationTeacherApplicationUser | null;
  sponsored_by_this_organization: boolean;
  status: string;
  updated_at: string;
};
