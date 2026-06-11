"use client";

import { AlertTriangle, CheckCircle, CreditCard, Loader2, RefreshCw, Trophy } from "lucide-react";
import Link from "next/link";
import { useMemo } from "react";
import { type RewardHistoryEntry, type WalletSummary } from "@/lib/learner";
import styles from "../learner-routes.module.css";
import { EmptyState } from "./EmptyState";
import { StatusPill } from "./StatusPill";
import { SummaryCard } from "./SummaryCard";
import { WalletActivityCard } from "./WalletActivityCard";
import { isWalletCreditPending } from "./isWalletCreditPending";
import { summarizeRewards } from "./summarizeRewards";

export function WalletContent({
  linking,
  onLinkWallet,
  onRefresh,
  rewards,
  wallet,
}: {
  linking: boolean;
  onLinkWallet: () => void;
  onRefresh: () => void;
  rewards: RewardHistoryEntry[];
  wallet: WalletSummary | null;
}) {
  const summary = useMemo(() => summarizeRewards(rewards), [rewards]);
  const pendingCreditCount = rewards.filter(isWalletCreditPending).length;
  const walletScope = wallet?.organization_id ? "Organization wallet" : "Personal wallet";

  const metricsSection = (
    <section className={`${styles.dashboardGrid} ${styles.walletGrid}`}>
      <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet state" value={wallet ? "Linked" : "Unlinked"} />
      <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Available value" value={wallet?.value || "0"} />
      <SummaryCard icon={<CheckCircle size={20} aria-hidden />} label="Wallet credits" value={summary.credited} />
      <SummaryCard icon={<AlertTriangle size={20} aria-hidden />} label="Pending credits" value={pendingCreditCount} />
    </section>
  );

  const walletSummarySection = (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <h2>Wallet summary</h2>
        <button className={styles.iconAction} aria-label="Refresh wallet" type="button" onClick={onRefresh}>
          <RefreshCw size={18} aria-hidden />
        </button>
      </div>
      {wallet ? (
        <article className={styles.itemCard}>
          <div className={styles.itemHeader}>
            <h3>{walletScope}</h3>
            <StatusPill label="Ready for credits" tone="good" />
          </div>
          <p className={styles.muted}>Approved rewards can be credited here. Deposits and retirements are not available in this UI yet.</p>
          <strong className={styles.walletValue}>{wallet.value}</strong>
          <div className={styles.metaRow}>
            <span>{wallet.owner_type}</span>
            <span>{walletScope}</span>
            <span>{summary.needsHelp ? "Needs review" : "No wallet issues shown"}</span>
          </div>
          <div className={styles.actionRow}>
            <Link className={styles.secondaryLink} href="/rewards">
              View rewards
            </Link>
          </div>
        </article>
      ) : (
        <EmptyState
          action={
            <button className={styles.primaryLink} disabled={linking} onClick={onLinkWallet} type="button">
              {linking ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CreditCard size={18} aria-hidden />}
              Link wallet
            </button>
          }
          detail="A RustLearn wallet is required before approved rewards can be credited."
          title="Wallet not linked"
        />
      )}
    </section>
  );

  return (
    <>
      {wallet ? metricsSection : walletSummarySection}
      {wallet ? walletSummarySection : metricsSection}

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Reward credit history</h2>
          <StatusPill label={`${rewards.length} recent`} tone={rewards.length ? "good" : "neutral"} />
        </div>
        {rewards.length ? (
          <div className={styles.itemGrid}>
            {rewards.slice(0, 6).map((reward) => (
              <WalletActivityCard key={reward.reward_candidate_id} reward={reward} />
            ))}
          </div>
        ) : (
          <EmptyState
            actionHref="/courses"
            actionLabel="Open courses"
            detail="Reward credits appear after eligible course activity is reviewed and credited."
            title="No wallet activity yet"
          />
        )}
      </section>
    </>
  );
}
