"use client";

import { type OrganizationWalletAudit } from "@/lib/organization/OrganizationWalletAudit";
import { OrganizationWalletRewardCard } from "@/components/organization-routes/OrganizationWalletRewardCard";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { walletAuditSummary } from "../model/walletAuditSummary";

export function RewardCreditAuditPanel({ audit }: { audit: OrganizationWalletAudit }) {
  const summary = walletAuditSummary(audit);

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <h2>Reward credit audit</h2>
        <StatusPill label={`${summary.attentionRecords.length} attention`} tone={summary.attentionRecords.length ? "warn" : "good"} />
      </div>
      {audit.reward_records.length ? (
        <div className={styles.reportGrid}>
          {audit.reward_records.slice(0, 8).map((record) => (
            <OrganizationWalletRewardCard
              externalTransaction={audit.external_transactions.find(
                (external) => external.external_transaction_id === record.external_transaction_id,
              )}
              key={record.reward_candidate_id}
              record={record}
            />
          ))}
        </div>
      ) : (
        <p className={styles.muted}>No reward credit rows are associated with this organization wallet yet.</p>
      )}
    </section>
  );
}
