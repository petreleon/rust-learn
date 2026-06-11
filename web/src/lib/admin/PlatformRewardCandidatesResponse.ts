import { type PlatformRewardCandidateItem } from "./PlatformRewardCandidateItem";
import { type PlatformRewardCandidatePermissions } from "./PlatformRewardCandidatePermissions";

export type PlatformRewardCandidatesResponse = {
  candidates: PlatformRewardCandidateItem[];
  limit: number;
  offset: number;
  operator_permissions: PlatformRewardCandidatePermissions;
  search: string | null;
  status: string | null;
  total: number;
};
