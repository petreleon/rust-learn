export type AssessmentQuestionItem = {
  assessment_id: number;
  id: number;
  options: unknown | null;
  order: number;
  points: number;
  question_type: string;
  text: string;
};

export type AssessmentItem = {
  course_id: number;
  created_at: string;
  description: string | null;
  id: number;
  max_attempts: number;
  passing_score: number;
  published: boolean;
  questions: AssessmentQuestionItem[];
  title: string;
  updated_at: string;
};
