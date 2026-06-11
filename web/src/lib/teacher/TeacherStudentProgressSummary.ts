export type TeacherStudentProgressSummary = {
  completed_content_count: number | null;
  completion_percentage: number | null;
  last_activity_at: string | null;
  note: string;
  supported: boolean;
  total_content_count: number;
};
