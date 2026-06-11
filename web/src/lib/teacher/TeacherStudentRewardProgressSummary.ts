import { type TeacherStudentRewardCandidateSummary } from "./TeacherStudentRewardCandidateSummary";

export type TeacherStudentRewardProgressSummary = {
  completed_count: number;
  failed_count: number;
  latest_candidate: TeacherStudentRewardCandidateSummary | null;
  pending_teacher_count: number;
  reward_candidate_count: number;
  teacher_approved_count: number;
  teacher_rejected_count: number;
};
