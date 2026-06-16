export type TeacherAssessmentQuestion = {
  id: number;
  assessment_id: number;
  text: string;
  question_type: string;
  options: unknown | null;
  correct_answer: string | null;
  points: number;
  order: number;
};

export type TeacherAssessment = {
  id: number;
  course_id: number;
  title: string;
  description: string | null;
  passing_score: number;
  max_attempts: number;
  published: boolean;
  created_at: string;
  updated_at: string;
  questions: TeacherAssessmentQuestion[];
};
