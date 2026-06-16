"use client";

import { type PlatformCsvReport } from "@/lib/admin";

export const exportReports: Array<{
  description: string;
  label: string;
  report: PlatformCsvReport;
}> = [
  {
    description: "Top-level platform counts for users, organizations, courses, wallets, and notifications.",
    label: "Platform summary",
    report: "summary",
  },
  {
    description: "Teacher applications, amount approvals, payout failures, and reconciliation mismatches.",
    label: "Reward dashboard",
    report: "reward_dashboard",
  },
  {
    description: "Active fraud blocks grouped by scope and recent block records.",
    label: "Fraud dashboard",
    report: "fraud_dashboard",
  },
  {
    description: "Teacher application rows with requested scope, sponsor, reviewer, reason, and timestamps.",
    label: "Teacher applications",
    report: "teacher_applications",
  },
  {
    description: "Teacher and amount approval decisions for reward candidates.",
    label: "Reward approvals",
    report: "reward_approvals",
  },
  {
    description: "Token payout records and external transaction state.",
    label: "Token payouts",
    report: "token_payouts",
  },
  {
    description: "Wallet credit rows linked to internal transactions and notifications.",
    label: "Wallet credits",
    report: "wallet_credits",
  },
  {
    description: "Delegated permission grants with scope, expiration, revocation, and usage timestamps.",
    label: "Delegations",
    report: "delegated_permissions",
  },
];
