"use client";

import { ExternalLink } from "lucide-react";
import { type OrganizationWalletAudit, type OrganizationWalletRewardRecordAudit } from "@/lib/organization";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { Metric } from "./Metric";
import { StatusPill } from "./StatusPill";
import { formatDateTime } from "./formatDateTime";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { formatTokenAmount } from "./formatTokenAmount";
import { shortHash } from "./shortHash";
import { tokenExplorerUrl } from "./tokenExplorerUrl";
import { walletRewardNeedsAttention } from "./walletRewardNeedsAttention";

export function OrganizationWalletRewardCard({
  externalTransaction,
  record,
}: {
  externalTransaction?: OrganizationWalletAudit["external_transactions"][number];
  record: OrganizationWalletRewardRecordAudit;
}) {
  const needsAttention = walletRewardNeedsAttention(record.reconciliation_status);
  const explorerUrl = externalTransaction ? tokenExplorerUrl(externalTransaction) : null;
  return (
    <article className={styles.reportCard}>
      <div className={styles.sectionHeader}>
        <h3>{formatUnderscoreLabel(record.candidate_status)}</h3>
        <StatusPill label={formatUnderscoreLabel(record.reconciliation_status)} tone={needsAttention ? "warn" : "good"} />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Approved amount" value={formatTokenAmount(record.approved_amount)} />
        <Metric label="Wallet credit" value={record.wallet_credit_record_id ? "Recorded" : "Missing"} />
        <Metric label="Updated" value={formatDateTime(record.updated_at)} />
      </div>
      {explorerUrl && externalTransaction?.transaction_hash ? (
        <a
          className={styles.secondaryLink}
          href={explorerUrl}
          rel="noreferrer"
          target="_blank"
        >
          <ExternalLink size={17} aria-hidden />
          Token tx {shortHash(externalTransaction.transaction_hash)}
        </a>
      ) : externalTransaction?.transaction_hash ? (
        <span className={styles.permissionChip}>
          Token tx {shortHash(externalTransaction.transaction_hash)}
        </span>
      ) : externalTransaction ? (
        <span className={styles.permissionChip}>Token transfer recorded</span>
      ) : (
        <span className={styles.permissionChip}>No token link yet</span>
      )}
    </article>
  );
}
