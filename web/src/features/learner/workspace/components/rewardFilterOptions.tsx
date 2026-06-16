"use client";

import { type RewardStatusFilter } from "./RewardStatusFilter";

export const rewardFilterOptions: Array<{ label: string; value: RewardStatusFilter }> = [
  { label: "All", value: "all" },
  { label: "Pending", value: "pending_teacher_approval" },
  { label: "Approved", value: "amount_approved" },
  { label: "Processing", value: "token_pending" },
  { label: "Confirmed", value: "token_confirmed" },
  { label: "Credited", value: "wallet_credited" },
  { label: "Needs help", value: "needs_reconciliation" },
  { label: "Failed", value: "failed" },
];
