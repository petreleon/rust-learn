"use client";

import { type TeacherApplicationDashboardSummary } from "@/lib/organization/TeacherApplicationDashboardSummary";
import { Metric } from "@/features/organization/shared/route-kit/Metric";
import { StatusPill } from "@/features/organization/shared/route-kit/StatusPill";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function SponsoredApplicationsPanel({
  summary,
}: {
  summary: TeacherApplicationDashboardSummary;
}) {
  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <h2>Sponsored teacher applications</h2>
        <StatusPill label={`${summary.total} total`} tone="neutral" />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Submitted" value={summary.submitted} />
        <Metric label="Needs changes" value={summary.needs_changes} />
        <Metric label="Approved" value={summary.approved} />
      </div>
      <p className={styles.muted}>Sponsored application rows and decision history are not part of this report contract yet.</p>
    </section>
  );
}
