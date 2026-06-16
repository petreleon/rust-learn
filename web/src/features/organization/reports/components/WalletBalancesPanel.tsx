"use client";

import { type OrganizationWalletBalanceRow } from "@/lib/organization/OrganizationWalletBalanceRow";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import { formatTokenAmount } from "@/components/organization-routes/formatTokenAmount";
import styles from "@/components/organization-routes.module.css";

export function WalletBalancesPanel({ wallets }: { wallets: OrganizationWalletBalanceRow[] }) {
  const hasWalletRows = wallets.length > 0;

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <h2>Wallet balances</h2>
        <StatusPill label={`${wallets.length} wallet${wallets.length === 1 ? "" : "s"}`} tone={hasWalletRows ? "good" : "neutral"} />
      </div>
      {hasWalletRows ? (
        <div className={styles.compactList}>
          {wallets.map((wallet, index) => (
            <article className={styles.compactRow} key={wallet.wallet_id}>
              <span>Organization wallet {index + 1}</span>
              <strong>{formatTokenAmount(wallet.balance)}</strong>
            </article>
          ))}
        </div>
      ) : (
        <p className={styles.muted}>No organization wallet balances are attached to this report.</p>
      )}
    </section>
  );
}
