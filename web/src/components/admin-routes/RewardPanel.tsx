"use client";

import { Landmark } from "lucide-react";
import { platformCapabilityEnabled, type PlatformAdminWorkspace, type PlatformRewardDashboard } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "./EmptyState";
import { GatedPanel } from "./GatedPanel";
import { MetricCard } from "./MetricCard";
import { PanelError } from "./PanelError";
import { PanelLoading } from "./PanelLoading";
import { RiskList } from "./RiskList";
import { StatusPill } from "./StatusPill";
import { formatDate } from "./formatDate";
import { formatNumber } from "./formatNumber";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function RewardPanel({
  canApproveRewardAmount,
  canViewRewardAudit,
  dashboard,
  error,
  onRetry,
  state,
  workspace,
}: {
  canApproveRewardAmount: boolean;
  canViewRewardAudit: boolean;
  dashboard: PlatformRewardDashboard | null;
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
          key: "reward_amount_review",
          label: "Reward audit",
          permissions: ["VIEW_REWARD_AUDIT"],
        }}
        icon={<Landmark size={20} aria-hidden />}
        title="Reward operations unavailable"
      />
    );
  }

  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading reward operations" />;
  }

  if (state === "error" || !dashboard) {
    return <PanelError error={error} onRetry={onRetry} title="Reward operations failed" />;
  }

  const rewardStatuses = [
    ["Teacher pending", dashboard.reward_candidates.pending_teacher_approval],
    ["Teacher approved", dashboard.reward_candidates.teacher_approved],
    ["Amount approved", dashboard.reward_candidates.amount_approved],
    ["Token pending", dashboard.reward_candidates.token_pending],
    ["Wallet credited", dashboard.reward_candidates.wallet_credited],
    ["Needs reconciliation", dashboard.reward_candidates.needs_reconciliation],
    ["Failed", dashboard.reward_candidates.failed],
  ];

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Landmark size={20} aria-hidden />
        <div>
          <h2>Reward operations</h2>
          <p>Teacher approval, platform amount review, payout failures, and reconciliation stay separated.</p>
        </div>
        <StatusPill label={canApproveRewardAmount ? "Amount approval enabled" : "Audit only"} tone={canApproveRewardAmount ? "good" : "neutral"} />
      </div>

      <div className={styles.metricGrid}>
        <MetricCard label="Teacher applications submitted" value={dashboard.teacher_applications.submitted} />
        <MetricCard label="Pending amount approval" value={dashboard.pending_amount_approval_count} />
        <MetricCard label="Payout failures" value={dashboard.payout_failure_count} tone={dashboard.payout_failure_count ? "warn" : "neutral"} />
        <MetricCard label="Reconciliation mismatches" value={dashboard.reconciliation_mismatch_count} tone={dashboard.reconciliation_mismatch_count ? "warn" : "neutral"} />
      </div>

      <div className={styles.statusList}>
        {rewardStatuses.map(([label, value]) => (
          <div className={styles.statusRow} key={label}>
            <span>{label}</span>
            <strong>{formatNumber(value as number)}</strong>
          </div>
        ))}
      </div>

      <div className={styles.subsectionHeader}>
        <h3>Amount-ready candidates</h3>
        <StatusPill label={`${dashboard.pending_amount_approvals.length} recent`} />
      </div>
      {dashboard.pending_amount_approvals.length ? (
        <div className={styles.rowList}>
          {dashboard.pending_amount_approvals.slice(0, 6).map((candidate) => (
            <article className={styles.compactRow} key={candidate.reward_candidate_id}>
              <div>
                <strong>Candidate {candidate.reward_candidate_id}</strong>
                <span>
                  Course {candidate.course_id} · Student {candidate.student_user_id} · {formatUnderscoreLabel(candidate.event_type)}
                </span>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label={formatUnderscoreLabel(candidate.status)} tone="warn" />
                <span>{formatDate(candidate.updated_at)}</span>
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No teacher-approved candidates are waiting for platform amount review." />
      )}

      <RiskList dashboard={dashboard} />
      {!platformCapabilityEnabled(workspace, "teacher_applications") ? (
        <p className={styles.muted}>Teacher application review is gated for this session.</p>
      ) : null}
    </section>
  );
}
