import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";
import { type RewardCandidateAmountDecisionStatus } from "@/lib/admin/RewardCandidateAmountDecisionStatus";
import { candidateIsFinal } from "./rewardCandidateDisplay";

export type RewardAmountDecisionDraft = {
  amount: string;
  reason: string;
  status: RewardCandidateAmountDecisionStatus;
};

export const defaultRewardAmountDecisionDraft: RewardAmountDecisionDraft = {
  amount: "",
  reason: "",
  status: "approved",
};

export function canSubmitRewardAmountDecision({
  canApprove,
  candidate,
  draft,
  state,
}: {
  canApprove: boolean;
  candidate: PlatformRewardCandidateItem;
  draft: RewardAmountDecisionDraft;
  state: string;
}) {
  if (state === "submitting" || candidateIsFinal(candidate) || !draft.reason.trim()) return false;
  if (draft.status === "rejected") return true;
  return canApprove && isValidApprovedAmount(draft.amount);
}

export function isValidApprovedAmount(amount: string) {
  const trimmed = amount.trim();
  return trimmed.length > 0 && !Number.isNaN(Number(trimmed)) && Number(trimmed) >= 0;
}
