"use client";

import { ShieldAlert } from "lucide-react";
import { platformCapabilityEnabled, type PlatformAdminWorkspace, type PlatformFraudDashboard } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { EmptyState } from "./EmptyState";
import { GatedPanel } from "./GatedPanel";
import { MetricCard } from "./MetricCard";
import { PanelError } from "./PanelError";
import { PanelLoading } from "./PanelLoading";
import { StatusPill } from "./StatusPill";
import { formatDate } from "./formatDate";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { targetLabel } from "./targetLabel";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function FraudPanel({
  canManageFraud,
  canViewRewardAudit,
  dashboard,
  error,
  onRetry,
  state,
  workspace,
}: {
  canManageFraud: boolean;
  canViewRewardAudit: boolean;
  dashboard: PlatformFraudDashboard | null;
  error: RouteError | null;
  onRetry: () => void;
  state: SectionState;
  workspace: PlatformAdminWorkspace;
}) {
  if (!canViewRewardAudit) {
    return (
      <GatedPanel
        capability={{
          enabled: false,
          key: "fraud_blocks",
          label: "Fraud dashboard",
          permissions: ["VIEW_REWARD_AUDIT"],
        }}
        icon={<ShieldAlert size={20} aria-hidden />}
        title="Fraud dashboard unavailable"
      />
    );
  }

  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading fraud controls" />;
  }

  if (state === "error" || !dashboard) {
    return <PanelError error={error} onRetry={onRetry} title="Fraud controls failed" />;
  }

  const scopeRows = [
    ["Teachers", dashboard.active_by_scope.teacher],
    ["Organizations", dashboard.active_by_scope.organization],
    ["Courses", dashboard.active_by_scope.course],
    ["Reward policies", dashboard.active_by_scope.reward_policy],
  ];

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldAlert size={20} aria-hidden />
        <div>
          <h2>Fraud controls</h2>
          <p>Active blocks are separated by target scope and keep reason/evidence context visible.</p>
        </div>
        <StatusPill label={canManageFraud ? "Manage enabled" : "Audit only"} tone={canManageFraud ? "good" : "neutral"} />
      </div>

      <div className={styles.metricGrid}>
        <MetricCard label="Active blocks" value={dashboard.active_total} tone={dashboard.active_total ? "warn" : "neutral"} />
        {scopeRows.map(([label, value]) => (
          <MetricCard key={label} label={label as string} value={value as number} />
        ))}
      </div>

      <div className={styles.subsectionHeader}>
        <h3>Recent active blocks</h3>
        <StatusPill label={`${dashboard.active_blocks.length} active`} tone={dashboard.active_blocks.length ? "warn" : "good"} />
      </div>
      {dashboard.active_blocks.length ? (
        <div className={styles.rowList}>
          {dashboard.active_blocks.slice(0, 7).map((block) => (
            <article className={styles.compactRow} key={block.id}>
              <div>
                <strong>{formatUnderscoreLabel(block.scope_type)} block {block.id}</strong>
                <span>{block.reason}</span>
                <small>{targetLabel(block)}</small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label={block.expires_at ? `Expires ${formatDate(block.expires_at)}` : "No expiry"} tone={block.expires_at ? "neutral" : "warn"} />
                {block.evidence_reference ? <span>Evidence: {block.evidence_reference}</span> : null}
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No active fraud blocks are currently visible." />
      )}
      {!platformCapabilityEnabled(workspace, "fraud_blocks") ? (
        <p className={styles.muted}>Fraud block create and revoke actions are gated for this session.</p>
      ) : null}
    </section>
  );
}
