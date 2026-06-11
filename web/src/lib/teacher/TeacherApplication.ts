import { type TeacherApplicationScope } from "./TeacherApplicationScope";
import { type TeacherApplicationStatus } from "./TeacherApplicationStatus";

export type TeacherApplication = {
  applicant_user_id: number;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  experience_summary: string;
  id: number;
  idempotency_key: string | null;
  organization_sponsor_id: number | null;
  portfolio_links: string[];
  requested_course_id: number | null;
  requested_organization_id: number | null;
  requested_scope: TeacherApplicationScope;
  reviewer_id: number | null;
  status: TeacherApplicationStatus;
  updated_at: string;
};
