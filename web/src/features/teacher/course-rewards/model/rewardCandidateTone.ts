"use client";
export function rewardCandidateTone(status: string) {
  if (
    status === "teacher_approved" ||
    status === "amount_approved" ||
    status === "token_confirmed" ||
    status === "wallet_credited" ||
    status === "completed"
  ) {
    return "good";
  }

  if (
    status === "pending_teacher_approval" ||
    status === "teacher_rejected" ||
    status === "amount_rejected" ||
    status === "needs_reconciliation" ||
    status === "failed"
  ) {
    return "warn";
  }

  return "neutral";
}
