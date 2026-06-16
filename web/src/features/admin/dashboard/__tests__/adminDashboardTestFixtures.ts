import { type PlatformFraudDashboard } from "@/lib/admin/PlatformFraudDashboard";
import { type PlatformRewardDashboard } from "@/lib/admin/PlatformRewardDashboard";
import { type PlatformSystemStatus } from "@/lib/admin/PlatformSystemStatus";
import { type CurrentSession } from "@/lib/session/CurrentSession";

export function adminSession(permissions: string[]): CurrentSession {
  return {
    access: {
      learner: true,
      teacher: false,
      teacher_application: false,
      organization: false,
      platform_admin: permissions.length > 0,
    },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [
        { enabled: permissions.includes("VIEW_REPORT"), key: "summary", label: "Platform summary", permissions: ["VIEW_REPORT"] },
        { enabled: permissions.includes("VIEW_REWARD_AUDIT"), key: "reward_amount_review", label: "Reward audit", permissions: ["VIEW_REWARD_AUDIT"] },
        { enabled: permissions.includes("MANAGE_REWARD_FRAUD_BLOCKS"), key: "fraud_blocks", label: "Fraud blocks", permissions: ["MANAGE_REWARD_FRAUD_BLOCKS"] },
        { enabled: permissions.includes("EXPORT_DATA"), key: "exports", label: "Exports", permissions: ["EXPORT_DATA"] },
      ],
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: {
      email: "admin@example.com",
      email_verified: true,
      id: 1,
      kyc_verified: true,
      name: "Admin User",
    },
  };
}

export function rewardDashboard(): PlatformRewardDashboard {
  return {
    payout_failure_count: 0,
    payout_failures: [],
    pending_amount_approval_count: 2,
    pending_amount_approvals: [],
    reconciliation_mismatch_count: 0,
    reconciliation_mismatches: [],
    reward_candidates: {
      amount_approved: 0,
      amount_rejected: 0,
      completed: 0,
      failed: 0,
      needs_reconciliation: 0,
      notified: 0,
      pending_teacher_approval: 1,
      teacher_approved: 2,
      teacher_rejected: 0,
      token_confirmed: 0,
      token_pending: 0,
      total: 3,
      wallet_credited: 0,
    },
    teacher_applications: {
      approved: 0,
      needs_changes: 0,
      rejected: 0,
      submitted: 4,
      total: 4,
    },
  };
}

export function fraudDashboard(): PlatformFraudDashboard {
  return {
    active_blocks: [],
    active_by_scope: {
      course: 0,
      organization: 1,
      reward_policy: 0,
      teacher: 0,
    },
    active_total: 1,
  };
}

export function systemStatus(): PlatformSystemStatus {
  return {
    liveness: { status: "ok" },
    readiness: {
      checks: [{ message: null, name: "database", status: "ok" }],
      status: "ready",
    },
  };
}
