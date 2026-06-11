"use client";

import { ArrowLeft, BookOpen, CreditCard, Loader2, RefreshCw, UserPlus, Users } from "lucide-react";
import Link from "next/link";
import { enabledOrganizationCapabilities, type OrganizationDashboardSummary, type OrganizationWorkspaceItem } from "@/lib/organization";
import styles from "../organization-routes.module.css";
import { ActionCard } from "./ActionCard";
import { DashboardAlertCard } from "./DashboardAlertCard";
import { DashboardErrorState } from "./DashboardErrorState";
import { Metric } from "./Metric";
import { PermissionGateList } from "./PermissionGateList";
import { PermissionPreview } from "./PermissionPreview";
import { StatusPill } from "./StatusPill";
import { SummaryCard } from "./SummaryCard";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { type DashboardLoadState } from "./DashboardLoadState";
import { type RouteError } from "./RouteError";
import { formatTokenAmount } from "./formatTokenAmount";

export function OrganizationDashboard({
  dashboard,
  dashboardError,
  dashboardLoadState,
  onRefreshDashboard,
  organization,
}: {
  dashboard: OrganizationDashboardSummary | null;
  dashboardError: RouteError | null;
  dashboardLoadState: DashboardLoadState;
  onRefreshDashboard: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  const enabledCapabilities = enabledOrganizationCapabilities(organization);
  const deniedCapabilities = organization.capabilities.filter((capability) => !capability.enabled);
  const pendingTeacherApplications = dashboard
    ? dashboard.teacher_applications.submitted + dashboard.teacher_applications.needs_changes
    : 0;
  const walletBalance = dashboard?.wallet.available ? formatTokenAmount(dashboard.wallet.balance_total) : "Gated";

  return (
    <>
      <section className={styles.workspaceHero}>
        <div className={styles.workspaceTitleBlock}>
          <Link className={styles.backLink} href="/organizations">
            <ArrowLeft size={17} aria-hidden />
            Organizations
          </Link>
          <p className={styles.eyebrow}>Selected workspace</p>
          <h2>{organization.name}</h2>
          <p className={styles.muted}>
            This dashboard combines the resolved session scope with the organization dashboard
            contract so operators can see attention items before opening member, course, report,
            wallet, or nomination workspaces.
          </p>
        </div>
        <div className={styles.actionRow}>
          <button className={styles.secondaryButton} onClick={onRefreshDashboard} type="button">
            <RefreshCw size={17} aria-hidden />
            Refresh
          </button>
        </div>
        <div className={styles.permissionRow}>
          {organization.roles.length ? (
            organization.roles.map((role) => <span className={styles.permissionChip} key={role}>{role}</span>)
          ) : (
            <span className={styles.permissionChip}>No role label</span>
          )}
          {dashboard ? (
            <StatusPill label={formatUnderscoreLabel(dashboard.health.status)} tone={dashboard.health.status === "attention" ? "warn" : "good"} />
          ) : null}
        </div>
      </section>

      {dashboardLoadState === "loading" || dashboardLoadState === "idle" ? (
        <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
          <div className={styles.panelHeader}>
            <Loader2 className={styles.spin} size={20} aria-hidden />
            <h2>Loading dashboard summary</h2>
          </div>
          <div className={styles.skeletonGrid} aria-hidden>
            <div className={styles.skeleton} />
            <div className={styles.skeleton} />
            <div className={styles.skeleton} />
          </div>
        </section>
      ) : null}

      {dashboardLoadState === "error" ? (
        <DashboardErrorState error={dashboardError} onRetry={onRefreshDashboard} />
      ) : null}

      {dashboard ? (
        <>
          <section className={styles.summaryGrid}>
            <SummaryCard icon={<Users size={20} aria-hidden />} label="Members" value={dashboard.members.available ? dashboard.members.total : "Gated"} />
            <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Courses" value={dashboard.courses.available ? dashboard.courses.total : "Gated"} />
            <SummaryCard icon={<UserPlus size={20} aria-hidden />} label="Pending teacher apps" value={dashboard.teacher_applications.available ? pendingTeacherApplications : "Gated"} />
            <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet balance" value={walletBalance} />
          </section>

          <section className={styles.twoColumn}>
            <section className={styles.panel}>
              <div className={styles.sectionHeader}>
                <h2>Attention items</h2>
                <StatusPill label={`${dashboard.health.alert_count} visible`} tone={dashboard.health.status === "attention" ? "warn" : "good"} />
              </div>
              <div className={styles.compactList}>
                {dashboard.alerts.map((alert) => (
                  <DashboardAlertCard alert={alert} key={`${alert.kind}-${alert.message}`} />
                ))}
              </div>
            </section>

            <section className={styles.panel}>
              <div className={styles.sectionHeader}>
                <h2>Operational signals</h2>
                <StatusPill label={dashboard.rewards.available ? "Reports visible" : "Reports gated"} tone={dashboard.rewards.available ? "good" : "warn"} />
              </div>
              <div className={styles.metricGrid}>
                <Metric label="Published courses" value={dashboard.courses.available ? dashboard.courses.published : "Gated"} />
                <Metric label="Reward candidates" value={dashboard.rewards.available ? dashboard.rewards.reward_candidate_count : "Gated"} />
                <Metric label="Approved amount" value={dashboard.rewards.available ? formatTokenAmount(dashboard.rewards.approved_amount_total) : "Gated"} />
              </div>
              <div className={styles.metricGrid}>
                <Metric label="Verified members" value={dashboard.members.available ? dashboard.members.verified_email_count : "Gated"} />
                <Metric label="Needs changes" value={dashboard.courses.available ? dashboard.courses.needs_changes : "Gated"} />
                <Metric label="Wallets" value={dashboard.wallet.available ? dashboard.wallet.wallet_count : "Gated"} />
              </div>
              <PermissionGateList dashboard={dashboard} />
            </section>
          </section>
        </>
      ) : null}

      <section className={styles.twoColumn}>
        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Available quick actions</h2>
            <StatusPill label={`${enabledCapabilities.length} enabled`} tone={enabledCapabilities.length ? "good" : "neutral"} />
          </div>
          {enabledCapabilities.length ? (
            <div className={styles.actionGrid}>
              {enabledCapabilities.map((capability) => (
                <ActionCard capability={capability} enabled key={capability.key} organizationId={organization.id} />
              ))}
            </div>
          ) : (
            <p className={styles.muted}>
              Your session can see this organization, but no management capability is enabled.
            </p>
          )}
        </section>

        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Permission preview</h2>
            <StatusPill label={`${organization.effectivePermissionCount} effective`} tone="neutral" />
          </div>
          <PermissionPreview organization={organization} />
        </section>
      </section>

      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Denied actions</h2>
          <StatusPill label={`${deniedCapabilities.length} gated`} tone={deniedCapabilities.length ? "warn" : "good"} />
        </div>
        <div className={styles.actionGrid}>
          {deniedCapabilities.map((capability) => (
            <ActionCard capability={capability} enabled={false} key={capability.key} organizationId={organization.id} />
          ))}
        </div>
      </section>
    </>
  );
}
