import { type TeacherRewardCandidateDecisionStatus } from "./TeacherRewardCandidateDecisionStatus";

export type DecideTeacherRewardCandidatePayload = {
  decision_reason?: string | null;
  status: TeacherRewardCandidateDecisionStatus;
};
