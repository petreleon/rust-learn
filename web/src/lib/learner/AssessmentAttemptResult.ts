export type AssessmentAttempt = {
  assessment_id: number;
  completed_at: string | null;
  id: number;
  passed: boolean | null;
  score: number | null;
  started_at: string;
  user_id: number;
};

export type AssessmentRewardHandoff = {
  candidate_id: number | null;
  event_type: string;
  message: string;
  policy_id: number | null;
  status: "not_earned" | "created" | "already_exists" | "missing_policy" | "failed";
};

export type AssessmentAttemptResult = {
  attempt: AssessmentAttempt;
  passed: boolean;
  percentage: number;
  reward_handoff: AssessmentRewardHandoff;
  score: number;
  total_points: number;
};
