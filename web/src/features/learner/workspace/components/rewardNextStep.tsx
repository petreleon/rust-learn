"use client";

import { type RewardHistoryEntry } from "@/lib/learner";

export function rewardNextStep(reward: RewardHistoryEntry) {
  switch (reward.status) {
    case "pending_teacher_approval":
      return "Waiting for teacher review.";
    case "teacher_approved":
      return "Teacher approved this activity; amount review is next.";
    case "teacher_rejected":
      return "Teacher rejected this activity. Check the course context before resubmitting.";
    case "amount_approved":
    case "adjusted":
      return "Amount is approved and token processing can continue.";
    case "amount_rejected":
      return "Amount review rejected this reward.";
    case "token_pending":
      return "Token transaction is being processed.";
    case "token_confirmed":
      return "Token transaction is confirmed; wallet credit is next.";
    case "wallet_credited":
    case "notified":
    case "completed":
      return "Reward was credited to the wallet.";
    case "needs_reconciliation":
      return "Reward needs reconciliation before it is final.";
    case "failed":
      return "Reward failed and needs support review.";
    default:
      return "Reward status changed after this page loaded.";
  }
}
