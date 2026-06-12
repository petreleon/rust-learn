import { type PlatformCapabilityKey } from "./PlatformCapabilityKey";

export const platformCapabilityDefinitions: Array<{
  key: PlatformCapabilityKey;
  label: string;
  permissions: string[];
}> = [
  {
    key: "summary",
    label: "Platform summary",
    permissions: ["VIEW_REPORT"],
  },
  {
    key: "kyc_reviews",
    label: "KYC review",
    permissions: ["REVIEW_KYC_SUBMISSIONS"],
  },
  {
    key: "teacher_applications",
    label: "Teacher review",
    permissions: ["REVIEW_TEACHER_APPLICATIONS", "APPROVE_TEACHER_APPLICATION", "REJECT_TEACHER_APPLICATION"],
  },
  {
    key: "reward_amount_review",
    label: "Reward amount review",
    permissions: ["APPROVE_REWARD_AMOUNT", "VIEW_REWARD_AUDIT"],
  },
  {
    key: "fraud_blocks",
    label: "Fraud controls",
    permissions: ["MANAGE_REWARD_FRAUD_BLOCKS", "BLOCK_REWARD_TEACHER", "BLOCK_REWARD_ORGANIZATION"],
  },
  {
    key: "delegations",
    label: "Delegations",
    permissions: ["MANAGE_ROLE_PERMISSIONS", "VIEW_ROLE_ASSIGNMENTS"],
  },
  {
    key: "exports",
    label: "Exports",
    permissions: ["EXPORT_DATA"],
  },
  {
    key: "wallets",
    label: "Wallet audit",
    permissions: ["MANAGE_WALLETS", "VIEW_TRANSACTIONS", "VIEW_SENSITIVE_TRANSACTIONS"],
  },
  {
    key: "system",
    label: "System status",
    permissions: ["VIEW_REPORT", "VIEW_AUDIT_LOGS", "VIEW_ANALYTICS_DASHBOARD", "MANAGE_TEST_SUITES"],
  },
];
