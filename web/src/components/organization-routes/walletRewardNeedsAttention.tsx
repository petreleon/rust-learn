"use client";
export function walletRewardNeedsAttention(status: string) {
  return !["reconciled", "closed_without_payout"].includes(status);
}
