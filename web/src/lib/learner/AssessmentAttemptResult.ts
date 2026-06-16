export type AssessmentAttempt = {
  assessment_id: number;
  completed_at: string | null;
  id: number;
  passed: boolean | null;
  score: number | null;
  started_at: string;
  user_id: number;
};

export type AssessmentAttemptResult = {
  attempt: AssessmentAttempt;
  passed: boolean;
  percentage: number;
  score: number;
  total_points: number;
};
