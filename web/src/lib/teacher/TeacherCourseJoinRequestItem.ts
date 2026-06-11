import { type TeacherEnrollmentUserSummary } from "./TeacherEnrollmentUserSummary";

export type TeacherCourseJoinRequestItem = {
  can_decide: boolean;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  id: number;
  requester: TeacherEnrollmentUserSummary;
  reviewer: TeacherEnrollmentUserSummary | null;
  status: string;
  updated_at: string;
};
