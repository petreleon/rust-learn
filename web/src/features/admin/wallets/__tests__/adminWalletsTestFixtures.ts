import { type PlatformReportSummary } from "@/lib/admin/PlatformReportSummary";
import { type PlatformWalletReconciliation } from "@/lib/admin/PlatformWalletReconciliation";
import { type CurrentSession } from "@/lib/session/CurrentSession";

export function adminWalletSession(permissions: string[]): CurrentSession {
  return {
    access: {
      learner: true,
      organization: false,
      platform_admin: permissions.length > 0,
      teacher: false,
      teacher_application: false,
    },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [
        {
          enabled: permissions.includes("VIEW_TRANSACTIONS"),
          key: "wallets",
          label: "Wallets",
          permissions: ["VIEW_TRANSACTIONS"],
        },
        {
          enabled: permissions.includes("EXPORT_DATA"),
          key: "exports",
          label: "Exports",
          permissions: ["EXPORT_DATA"],
        },
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

export function walletSummary(): PlatformReportSummary {
  return {
    total_courses: 7,
    total_notifications: 11,
    total_organizations: 3,
    total_users: 42,
    total_wallets: 12,
  };
}

export function walletReconciliation(): PlatformWalletReconciliation {
  return {
    total_external_transactions: 5,
    total_internal_transactions: 8,
    total_needs_reconciliation: 3,
    total_reward_records: 6,
    total_wallets: 12,
    wallets: [
      {
        balance: "150.00",
        external_transaction_count: 2,
        internal_transaction_count: 3,
        missing_credit_count: 1,
        missing_notification_count: 0,
        missing_payout_count: 0,
        needs_reconciliation_count: 1,
        organization_id: null,
        owner_type: "user",
        reward_record_count: 4,
        user_id: 44,
        wallet_id: 10,
      },
    ],
  };
}

export function walletTokenTaxes() {
  return {
    deposit: {
      operation: "deposit",
      tax_amount: "2",
    },
    retire: {
      operation: "retire",
      tax_amount: "1",
    },
  };
}
