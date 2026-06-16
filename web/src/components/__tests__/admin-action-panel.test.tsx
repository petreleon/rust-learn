import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { ActionPanel } from "@/features/admin/shared/route-kit/ActionPanel";
import { type PlatformAdminWorkspace } from "@/lib/admin/PlatformAdminWorkspace";
import { type PlatformCapabilityKey } from "@/lib/admin/PlatformCapabilityKey";
import { type PlatformFraudDashboard } from "@/lib/admin/PlatformFraudDashboard";
import { type PlatformRewardDashboard } from "@/lib/admin/PlatformRewardDashboard";
import { type PlatformSystemStatus } from "@/lib/admin/PlatformSystemStatus";

const capabilityKeys: PlatformCapabilityKey[] = [
  "kyc_reviews",
  "teacher_applications",
  "reward_amount_review",
  "fraud_blocks",
  "delegations",
  "exports",
  "wallets",
  "system",
];

function makeWorkspace(): PlatformAdminWorkspace {
  return {
    capabilities: capabilityKeys.map((key) => ({
      enabled: true,
      key,
      label: key,
      permissions: [],
    })),
    delegatedPermissionCount: 0,
    directPermissionCount: 7,
    effectivePermissionCount: 7,
    effectivePermissions: ["APPROVE_REWARD_AMOUNT", "EXPORT_DATA", "MANAGE_FRAUD_BLOCKS", "REVIEW_KYC_SUBMISSIONS"],
    roles: ["platform_admin"],
  };
}

describe("ActionPanel", () => {
  it("renders admin review lanes as route links", () => {
    render(
      <ActionPanel
        canApproveRewardAmount
        canExportData
        canManageFraud
        canViewRewardAudit
        fraudDashboard={{ active_total: 2 } as PlatformFraudDashboard}
        rewardDashboard={
          {
            pending_amount_approval_count: 4,
            teacher_applications: { submitted: 3 },
          } as PlatformRewardDashboard
        }
        systemStatus={
          {
            liveness: { status: "ok" },
            readiness: { status: "ready" },
          } as PlatformSystemStatus
        }
        workspace={makeWorkspace()}
      />,
    );

    expect(screen.getByRole("link", { name: "Open Teacher review" })).toHaveAttribute(
      "href",
      "/admin/teacher-applications",
    );
    expect(screen.getByRole("link", { name: "Open KYC review" })).toHaveAttribute("href", "/admin/kyc");
    expect(screen.getByRole("link", { name: "Open Amount review" })).toHaveAttribute(
      "href",
      "/admin/rewards/amount-review",
    );
    expect(screen.getByRole("link", { name: "Open Fraud controls" })).toHaveAttribute(
      "href",
      "/admin/fraud-blocks",
    );
    expect(screen.getByRole("link", { name: "Open Delegations" })).toHaveAttribute("href", "/admin/delegations");
    expect(screen.getByRole("link", { name: "Open Exports" })).toHaveAttribute("href", "/admin/exports");
    expect(screen.getByRole("link", { name: "Open Wallet audit" })).toHaveAttribute("href", "/admin/wallets");
    expect(screen.getByRole("link", { name: "Open System" })).toHaveAttribute("href", "/admin/system");
  });
});
