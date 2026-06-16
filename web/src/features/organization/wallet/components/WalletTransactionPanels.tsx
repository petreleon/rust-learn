"use client";

import { type OrganizationWalletAudit } from "@/lib/organization/OrganizationWalletAudit";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import { formatDateTime } from "@/components/organization-routes/formatDateTime";
import { formatTokenAmount } from "@/components/organization-routes/formatTokenAmount";
import { formatUnderscoreLabel } from "@/components/organization-routes/formatUnderscoreLabel";
import { shortHash } from "@/components/organization-routes/shortHash";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function WalletTransactionPanels({ audit }: { audit: OrganizationWalletAudit }) {
  return (
    <section className={styles.twoColumn}>
      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Internal ledger</h2>
          <StatusPill label={`${audit.internal_transactions.length} rows`} tone={audit.internal_transactions.length ? "good" : "neutral"} />
        </div>
        {audit.internal_transactions.length ? (
          <div className={styles.compactList}>
            {audit.internal_transactions.slice(0, 8).map((transaction) => (
              <article className={styles.compactRow} key={transaction.internal_transaction_id}>
                <span>
                  <StatusPill label={formatUnderscoreLabel(transaction.transaction_type)} tone="neutral" />
                  {formatDateTime(transaction.created_at)}
                </span>
                <strong>{formatTokenAmount(transaction.amount)}</strong>
              </article>
            ))}
          </div>
        ) : (
          <p className={styles.muted}>No internal ledger rows are available for this wallet yet.</p>
        )}
      </section>
      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Token transaction links</h2>
          <StatusPill label={`${audit.external_transactions.length} links`} tone={audit.external_transactions.length ? "good" : "neutral"} />
        </div>
        {audit.external_transactions.length ? (
          <div className={styles.compactList}>
            {audit.external_transactions.slice(0, 8).map((transaction) => (
              <article className={styles.compactRow} key={transaction.external_transaction_id}>
                <span>
                  <StatusPill label={transaction.event_type || "External transaction"} tone="neutral" />
                  {transaction.transaction_hash ? shortHash(transaction.transaction_hash) : transaction.blockchain_address}
                </span>
                <strong>{formatTokenAmount(transaction.amount)}</strong>
              </article>
            ))}
          </div>
        ) : (
          <p className={styles.muted}>No token payout or deposit transaction links are attached yet.</p>
        )}
      </section>
    </section>
  );
}
