export type TeacherAssessmentQuestionPayload = {
  text: string;
  question_type: string;
  options: unknown | null;
  correct_answer: string | null;
  points: number;
  order: number;
};

export type TeacherAssessmentPayload = {
  title: string;
  description: string | null;
  passing_score: number;
  max_attempts: number;
  published: boolean;
  questions: TeacherAssessmentQuestionPayload[];
};
