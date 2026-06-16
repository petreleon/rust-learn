"use client";

import { type RewardHistoryEntry } from "@/lib/learner";

export function isWalletCreditPending(reward: RewardHistoryEntry) {
  return !reward.wallet_credit && reward.status !== "failed" && reward.status !== "needs_reconciliation" && !reward.status.endsWith("_rejected");
}
