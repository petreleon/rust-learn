import { type PlatformCsvReport } from "./PlatformCsvReport";

export const platformCsvEndpoints: Record<PlatformCsvReport, { filename: string; path: string }> = {
  delegated_permissions: {
    filename: "platform-delegated-permissions.csv",
    path: "/reports/platform/delegated-permissions.csv",
  },
  fraud_dashboard: {
    filename: "platform-fraud-dashboard.csv",
    path: "/reports/platform/fraud-dashboard.csv",
  },
  reward_approvals: {
    filename: "platform-reward-approvals.csv",
    path: "/reports/platform/reward-approvals.csv",
  },
  reward_dashboard: {
    filename: "platform-reward-dashboard.csv",
    path: "/reports/platform/reward-dashboard.csv",
  },
  summary: {
    filename: "platform-summary.csv",
    path: "/reports/platform/summary.csv",
  },
  teacher_applications: {
    filename: "platform-teacher-applications.csv",
    path: "/reports/platform/teacher-applications.csv",
  },
  token_payouts: {
    filename: "platform-token-payouts.csv",
    path: "/reports/platform/token-payouts.csv",
  },
  wallet_credits: {
    filename: "platform-wallet-credits.csv",
    path: "/reports/platform/wallet-credits.csv",
  },
};
