import { decideRewardAmount } from "@/lib/admin/decideRewardAmount";
import { fetchPlatformRewardCandidates } from "@/lib/admin/fetchPlatformRewardCandidates";
import { fetchRewardCandidateAudit } from "@/lib/admin/fetchRewardCandidateAudit";
import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";
import { type PlatformRewardCandidatesResponse } from "@/lib/admin/PlatformRewardCandidatesResponse";
import { type RewardAuditEvent } from "@/lib/admin/RewardAuditEvent";
import { type RewardCandidateAmountDecisionStatus } from "@/lib/admin/RewardCandidateAmountDecisionStatus";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { ADMIN_REWARD_CANDIDATE_PAGE_SIZE } from "../model/rewardAmountPagination";

export function loadAdminRewardAmountSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function loadAdminRewardCandidates({
  offset,
  search,
  status,
  token,
}: {
  offset: number;
  search: string;
  status: string;
  token: string;
}): Promise<PlatformRewardCandidatesResponse> {
  return fetchPlatformRewardCandidates({
    limit: ADMIN_REWARD_CANDIDATE_PAGE_SIZE,
    offset,
    search,
    status,
    token,
  });
}

export function loadAdminRewardCandidateAudit({
  candidateId,
  token,
}: {
  candidateId: number;
  token: string;
}): Promise<RewardAuditEvent[]> {
  return fetchRewardCandidateAudit({ candidateId, token });
}

export function decideAdminRewardAmount({
  approvedAmount,
  candidateId,
  decisionReason,
  status,
  token,
}: {
  approvedAmount: string | null;
  candidateId: number;
  decisionReason: string;
  status: RewardCandidateAmountDecisionStatus;
  token: string;
}): Promise<PlatformRewardCandidateItem> {
  return decideRewardAmount({
    approvedAmount,
    candidateId,
    decisionReason,
    status,
    token,
  });
}
