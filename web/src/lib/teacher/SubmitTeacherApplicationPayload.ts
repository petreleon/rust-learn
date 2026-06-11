import { type TeacherApplicationScope } from "./TeacherApplicationScope";

export type SubmitTeacherApplicationPayload = {
  experience_summary: string;
  idempotency_key?: string;
  organization_sponsor_id?: number;
  portfolio_links?: string[];
  requested_course_id?: number;
  requested_organization_id?: number;
  requested_scope: TeacherApplicationScope;
};
