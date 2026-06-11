export type RewardExecutionFailureRow = {
  attempts: number;
  last_error: string | null;
  reward_candidate_id: number;
  reward_execution_job_id: number;
  status: string;
  updated_at: string;
};
