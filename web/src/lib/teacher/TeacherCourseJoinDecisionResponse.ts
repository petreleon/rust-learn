export type TeacherCourseJoinDecisionResponse = {
  course_id: number;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  id: number;
  requester_user_id: number;
  reviewer_user_id: number | null;
  status: string;
  updated_at: string;
};
