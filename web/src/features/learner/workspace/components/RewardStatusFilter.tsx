"use client";
export type RewardStatusFilter =
  | "all"
  | "pending_teacher_approval"
  | "amount_approved"
  | "token_pending"
  | "token_confirmed"
  | "wallet_credited"
  | "needs_reconciliation"
  | "failed";
