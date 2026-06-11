export type AssessmentAttemptResult = {
  attempt: {
    assessment_id: number;
    completed_at: string;
    id: number;
    passed: boolean;
    score: number;
    started_at: string;
    user_id: number;
  };
  passed: boolean;
  percentage: number;
  score: number;
  total_points: number;
};
