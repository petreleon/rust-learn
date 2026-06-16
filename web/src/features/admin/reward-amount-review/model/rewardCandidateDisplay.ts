import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";

export const rewardCandidateStatusOptions: Array<{ label: string; value: string }> = [
  { label: "All statuses", value: "" },
  { label: "Pending teacher approval", value: "pending_teacher_approval" },
  { label: "Teacher approved", value: "teacher_approved" },
  { label: "Amount approved", value: "amount_approved" },
  { label: "Amount rejected", value: "amount_rejected" },
  { label: "Token pending", value: "token_pending" },
  { label: "Token confirmed", value: "token_confirmed" },
  { label: "Wallet credited", value: "wallet_credited" },
  { label: "Needs reconciliation", value: "needs_reconciliation" },
  { label: "Failed", value: "failed" },
];

export function rewardCandidateStatusTone(status: string): "good" | "neutral" | "warn" {
  if (["amount_approved", "wallet_credited", "token_confirmed", "completed"].includes(status)) return "good";
  if (["teacher_approved", "pending_teacher_approval", "token_pending"].includes(status)) return "warn";
  if (["needs_reconciliation", "amount_rejected", "teacher_rejected", "failed"].includes(status)) return "warn";
  return "neutral";
}

export function candidateIsFinal(candidate: PlatformRewardCandidateItem) {
  return candidate.status !== "teacher_approved" && candidate.status !== "pending_teacher_approval";
}

export function countCandidatesByStatus(candidates: PlatformRewardCandidateItem[], status: string) {
  return candidates.filter((candidate) => candidate.status === status).length;
}
