"use client";
export function candidateStatusTone(status: string): "good" | "neutral" | "warn" {
  if (status === "amount_approved" || status === "wallet_credited" || status === "token_confirmed" || status === "completed") {
    return "good";
  }
  if (status === "teacher_approved" || status === "pending_teacher_approval" || status === "token_pending" || status === "needs_reconciliation") {
    return "warn";
  }
  if (status === "amount_rejected" || status === "teacher_rejected" || status === "failed") {
    return "warn";
  }
  return "neutral";
}
