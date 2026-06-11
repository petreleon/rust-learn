"use client";

import { type RewardHistoryEntry } from "@/lib/learner";

export function summarizeRewards(rewards: RewardHistoryEntry[]) {
  return rewards.reduce(
    (summary, reward) => {
      if (reward.status === "pending_teacher_approval") {
        summary.pendingTeacher += 1;
      }
      if (reward.status.includes("token") || reward.status === "amount_approved") {
        summary.processing += 1;
      }
      if (reward.wallet_credit) {
        summary.credited += 1;
      }
      if (reward.status === "failed" || reward.status === "needs_reconciliation") {
        summary.needsHelp += 1;
      }
      return summary;
    },
    {
      credited: 0,
      needsHelp: 0,
      pendingTeacher: 0,
      processing: 0,
    },
  );
}
