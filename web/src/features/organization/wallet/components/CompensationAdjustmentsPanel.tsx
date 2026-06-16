"use client";

import { type OrganizationWalletCompensationRecordAudit } from "@/lib/organization/OrganizationWalletCompensationRecordAudit";
import { Metric } from "@/features/organization/shared/route-kit/Metric";
import { StatusPill } from "@/features/organization/shared/route-kit/StatusPill";
import { formatDateTime } from "@/features/organization/shared/route-kit/formatDateTime";
import { formatTokenAmount } from "@/features/organization/shared/route-kit/formatTokenAmount";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function CompensationAdjustmentsPanel({
  records,
}: {
  records: OrganizationWalletCompensationRecordAudit[];
}) {
  if (!records.length) {
    return null;
  }

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <h2>Compensation adjustments</h2>
        <StatusPill label={`${records.length} adjustments`} tone="warn" />
      </div>
      <div className={styles.reportGrid}>
        {records.slice(0, 6).map((record) => (
          <article className={styles.reportCard} key={record.id}>
            <h3>{record.reason}</h3>
            <div className={styles.metricGrid}>
              <Metric label="Amount" value={formatTokenAmount(record.amount)} />
              <Metric label="Created" value={formatDateTime(record.created_at)} />
              <Metric label="Reward" value="Adjustment" />
            </div>
          </article>
        ))}
      </div>
    </section>
  );
}
