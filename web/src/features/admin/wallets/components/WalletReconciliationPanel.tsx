"use client";

import { Landmark } from "lucide-react";
import { type PlatformWalletReconciliation } from "@/lib/admin/PlatformWalletReconciliation";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { MetricCard } from "@/features/admin/shared/route-kit/MetricCard";
import { PanelError } from "@/features/admin/shared/route-kit/PanelError";
import { PanelLoading } from "@/features/admin/shared/route-kit/PanelLoading";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";

export function WalletReconciliationPanel({
  error,
  onRetry,
  reconciliation,
  state,
}: {
  error: RouteError | null;
  onRetry: () => void;
  reconciliation: PlatformWalletReconciliation | null;
  state: LoadState;
}) {
  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading reconciliation" />;
  }

  if (state === "error" || !reconciliation) {
    return <PanelError error={error} onRetry={onRetry} title="Reconciliation failed" />;
  }

  return (
    <section className={styles.panel} aria-label="Wallet reconciliation">
      <div className={styles.panelHeader}>
        <Landmark size={20} aria-hidden />
        <div>
          <h2>Wallet reconciliation</h2>
          <p>Internal ledger, reward records, and missing credits across all wallets.</p>
        </div>
      </div>
      <div className={styles.metricGrid}>
        <MetricCard label="Wallets" value={reconciliation.total_wallets} />
        <MetricCard label="Internal transactions" value={reconciliation.total_internal_transactions} />
        <MetricCard label="External transactions" value={reconciliation.total_external_transactions} />
        <MetricCard label="Reward records" value={reconciliation.total_reward_records} />
        <MetricCard
          label="Needs reconciliation"
          tone={reconciliation.total_needs_reconciliation ? "warn" : "good"}
          value={reconciliation.total_needs_reconciliation}
        />
      </div>
      <div className={styles.rowList}>
        {reconciliation.wallets.map((wallet) => (
          <article className={styles.compactRow} key={wallet.wallet_id}>
            <div>
              <strong>Wallet {wallet.wallet_id}</strong>
              <span>
                {wallet.owner_type}
                {wallet.user_id ? ` · User ${wallet.user_id}` : ""}
                {wallet.organization_id ? ` · Org ${wallet.organization_id}` : ""}
              </span>
              <small>
                Balance {wallet.balance} · {wallet.internal_transaction_count} internal · {wallet.external_transaction_count} external
              </small>
            </div>
            <div className={styles.rowMeta}>
              {wallet.needs_reconciliation_count ? (
                <StatusPill label={`${wallet.needs_reconciliation_count} issues`} tone="warn" />
              ) : (
                <StatusPill label="OK" tone="good" />
              )}
            </div>
          </article>
        ))}
      </div>
    </section>
  );
}
