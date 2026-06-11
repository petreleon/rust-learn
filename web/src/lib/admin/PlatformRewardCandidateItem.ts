import { type PlatformRewardCandidateCourse } from "./PlatformRewardCandidateCourse";
import { type PlatformRewardCandidateUser } from "./PlatformRewardCandidateUser";

export type PlatformRewardCandidateItem = {
  approved_amount: string | null;
  course: PlatformRewardCandidateCourse;
  created_at: string;
  event_type: string;
  id: number;
  source_organization_id: number | null;
  source_scope: string;
  status: string;
  student: PlatformRewardCandidateUser;
  submitter: PlatformRewardCandidateUser;
  teacher_approver: PlatformRewardCandidateUser | null;
  teacher_decision_reason: string | null;
  updated_at: string;
};
