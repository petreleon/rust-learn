"use client";

import { ProductShell, type ShellNotice } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { ActionPanel } from "@/features/admin/shared/route-kit/ActionPanel";
import { AdminDeniedState } from "@/features/admin/shared/route-kit/AdminDeniedState";
import { ExportPanel } from "@/features/admin/shared/route-kit/ExportPanel";
import { FraudPanel } from "@/features/admin/shared/route-kit/FraudPanel";
import { LoadingState } from "@/features/admin/shared/route-kit/LoadingState";
import { RewardPanel } from "@/features/admin/shared/route-kit/RewardPanel";
import { SessionErrorState } from "@/features/admin/shared/route-kit/SessionErrorState";
import { SignedOutState } from "@/features/admin/shared/route-kit/SignedOutState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { SummaryPanel } from "@/features/admin/shared/route-kit/SummaryPanel";
import { SystemPanel } from "@/features/admin/shared/route-kit/SystemPanel";
import { type AdminDashboardRouteController } from "../route/useAdminDashboardRoute";

function dashboardNotice(route: AdminDashboardRouteController): ShellNotice | null {
  if (route.csvState === "success" && route.csvFilename) {
    return {
      message: `${route.csvFilename} downloaded.`,
      title: "CSV ready",
      tone: "success",
    };
  }

  if (route.csvState === "error" && route.csvError) {
    return {
      message: route.csvError.message,
      title: "CSV export failed",
      tone: route.csvError.status === 403 ? "warn" : "error",
    };
  }

  return null;
}

export function AdminDashboardView({ route }: { route: AdminDashboardRouteController }) {
  return (
    <ProductShell
      activeNav="admin"
      description="Platform-level review, reward operations, fraud controls, exports, and runtime status."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={dashboardNotice(route)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : route.allowed ? "Admin access" : "Access gated"} />
          <StatusPill label={`${route.workspace.effectivePermissionCount} platform permissions`} />
          <StatusPill label={`${route.workspace.delegatedPermissionCount} delegated`} />
        </>
      }
      title="Platform admin dashboard"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed ? (
        <div className={styles.stack}>
          <SummaryPanel
            canViewSummary={route.canViewSummary}
            error={route.summaryError}
            onRetry={route.loadDashboard}
            state={route.summaryState}
            summary={route.summary}
            workspace={route.workspace}
          />
          <ActionPanel
            canApproveRewardAmount={route.canApproveRewards}
            canExportData={route.canExportData}
            canManageFraud={route.canManageFraud}
            canViewRewardAudit={route.canViewRewardOperations}
            fraudDashboard={route.fraudDashboard}
            rewardDashboard={route.rewardDashboard}
            systemStatus={route.systemStatus}
            workspace={route.workspace}
          />
          <div className={styles.twoColumn}>
            <RewardPanel
              canApproveRewardAmount={route.canApproveRewards}
              canViewRewardAudit={route.canViewRewardOperations}
              dashboard={route.rewardDashboard}
              error={route.rewardError}
              onRetry={route.loadDashboard}
              state={route.rewardState}
              workspace={route.workspace}
            />
            <FraudPanel
              canManageFraud={route.canManageFraud}
              canViewRewardAudit={route.canViewRewardOperations}
              dashboard={route.fraudDashboard}
              error={route.fraudError}
              onRetry={route.loadDashboard}
              state={route.fraudState}
              workspace={route.workspace}
            />
          </div>
          <div className={styles.twoColumn}>
            <ExportPanel
              canExportData={route.canExportData}
              csvState={route.csvState}
              onDownload={route.handleCsvDownload}
              workspace={route.workspace}
            />
            <SystemPanel
              error={route.systemError}
              onRetry={route.loadDashboard}
              state={route.systemState}
              status={route.systemStatus}
            />
          </div>
        </div>
      ) : null}
    </ProductShell>
  );
}
