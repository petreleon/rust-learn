"use client";

import { type PlatformRewardDashboard } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { EmptyState } from "./EmptyState";
import { StatusPill } from "./StatusPill";
import { formatDate } from "./formatDate";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";

export function RiskList({ dashboard }: { dashboard: PlatformRewardDashboard }) {
  const risks = [
    ...dashboard.payout_failures.slice(0, 3).map((failure) => ({
      detail: failure.last_error || "No backend error message was recorded.",
      key: `payout-${failure.reward_execution_job_id}`,
      label: `Payout job ${failure.reward_execution_job_id}`,
      meta: `Candidate ${failure.reward_candidate_id} · ${failure.attempts} attempts`,
      tone: "warn" as const,
      updatedAt: failure.updated_at,
    })),
    ...dashboard.reconciliation_mismatches.slice(0, 3).map((mismatch) => ({
      detail: formatUnderscoreLabel(mismatch.mismatch_type),
      key: `reconciliation-${mismatch.reward_candidate_id}`,
      label: `Candidate ${mismatch.reward_candidate_id}`,
      meta: `Course ${mismatch.course_id} · Student ${mismatch.student_user_id}`,
      tone: "warn" as const,
      updatedAt: mismatch.updated_at,
    })),
  ];

  return (
    <>
      <div className={styles.subsectionHeader}>
        <h3>Operational risks</h3>
        <StatusPill label={`${risks.length} visible`} tone={risks.length ? "warn" : "good"} />
      </div>
      {risks.length ? (
        <div className={styles.rowList}>
          {risks.map((risk) => (
            <article className={styles.compactRow} key={risk.key}>
              <div>
                <strong>{risk.label}</strong>
                <span>{risk.detail}</span>
                <small>{risk.meta}</small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label="Needs attention" tone={risk.tone} />
                <span>{formatDate(risk.updatedAt)}</span>
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No payout failures or reconciliation mismatches are visible." />
      )}
    </>
  );
}
