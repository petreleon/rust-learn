"use client";

import { type OrganizationWalletAudit } from "@/lib/organization/OrganizationWalletAudit";
import { Metric } from "@/components/organization-routes/Metric";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import { formatTokenAmount } from "@/components/organization-routes/formatTokenAmount";
import styles from "@/components/organization-routes.module.css";
import { walletAuditSummary } from "../model/walletAuditSummary";

export function WalletBudgetCoveragePanels({ audit }: { audit: OrganizationWalletAudit }) {
  const summary = walletAuditSummary(audit);

  return (
    <section className={styles.twoColumn}>
      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Budget constraints</h2>
          <StatusPill label={summary.budgetTone === "warn" ? "Review balance" : "Balance visible"} tone={summary.budgetTone} />
        </div>
        <div className={styles.metricGrid}>
          <Metric label="Approved amount" value={formatTokenAmount(summary.approvedAmountTotal)} />
          <Metric label="Uncredited approved" value={formatTokenAmount(summary.uncreditedAmountTotal)} />
          <Metric label="Balance" value={formatTokenAmount(summary.walletBalance)} />
        </div>
        <p className={styles.muted}>
          Compare visible balance with uncredited approved rewards before reward-budget actions. This view does not reserve funds
          or execute payouts; it gives operators the current audit context.
        </p>
      </section>
      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Audit coverage</h2>
          <StatusPill label={summary.hasAuditRows ? "Rows available" : "Empty audit"} tone={summary.hasAuditRows ? "good" : "neutral"} />
        </div>
        <div className={styles.metricGrid}>
          <Metric label="Internal ledger" value={audit.internal_transactions.length} />
          <Metric label="Token links" value={audit.external_transactions.length} />
          <Metric label="Compensations" value={audit.compensation_records.length} />
        </div>
        <p className={styles.muted}>
          Empty audit rows are valid for a newly linked wallet. Reward-credit and payout rows appear after reward execution records
          are created.
        </p>
      </section>
    </section>
  );
}
