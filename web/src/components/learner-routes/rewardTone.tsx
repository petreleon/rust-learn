"use client";
export function rewardTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "wallet_credited" || status === "completed" || status === "notified") {
    return "good";
  }
  if (status === "failed") {
    return "bad";
  }
  if (status === "needs_reconciliation" || status.endsWith("_rejected")) {
    return "warn";
  }
  return "neutral";
}
