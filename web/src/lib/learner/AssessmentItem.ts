export type AssessmentItem = {
  course_id: number;
  created_at: string;
  description: string | null;
  id: number;
  max_attempts: number;
  passing_score: number;
  published: boolean;
  title: string;
  updated_at: string;
};
