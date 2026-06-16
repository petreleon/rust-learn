"use client";

import { AlertTriangle, CheckCircle, CreditCard, Loader2, RefreshCw, ShieldCheck, Trophy } from "lucide-react";
import Link from "next/link";
import { useMemo } from "react";
import { type RewardHistoryEntry, type WalletDepositIntentAudit, type WalletSummary } from "@/lib/learner";
import { WalletBurnPanel } from "../wallet-burns/route/WalletBurnPanel";
import { WalletDepositIntentHistoryCard } from "../wallet-transfers/components/WalletDepositIntentHistoryCard";
import { WalletTransferPanel } from "../wallet-transfers/route/WalletTransferPanel";
import styles from "../learner-workspace.module.css";
import { EmptyState } from "./EmptyState";
import { StatusPill } from "./StatusPill";
import { SummaryCard } from "./SummaryCard";
import { WalletActivityCard } from "./WalletActivityCard";
import { isWalletCreditPending } from "./isWalletCreditPending";
import { summarizeRewards } from "./summarizeRewards";
import { walletKycGateCopy } from "./walletKycGate";

export function WalletContent({
  kycVerified,
  linking,
  onLinkWallet,
  onRefresh,
  rewards,
  wallet,
  walletHistory = [],
}: {
  kycVerified: boolean;
  linking: boolean;
  onLinkWallet: () => void;
  onRefresh: () => void;
  rewards: RewardHistoryEntry[];
  wallet: WalletSummary | null;
  walletHistory?: WalletDepositIntentAudit[];
}) {
  const summary = useMemo(() => summarizeRewards(rewards), [rewards]);
  const kycGate = walletKycGateCopy(kycVerified);
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
          <p className={styles.muted}>Approved rewards, deposit intents, and token retirements are managed from this wallet route.</p>
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
            <div className={styles.actionRow}>
              <button className={styles.primaryLink} disabled={!kycGate.ready || linking} onClick={onLinkWallet} type="button">
                {linking ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CreditCard size={18} aria-hidden />}
                Link wallet
              </button>
              {!kycGate.ready ? <Link className={styles.secondaryLink} href="/settings/account">Open verification</Link> : null}
            </div>
          }
          detail={kycGate.ready ? "A RustLearn wallet is required before approved rewards can be credited." : kycGate.actionDetail}
          title="Wallet not linked"
        />
      )}
    </section>
  );

  const kycSection = (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <h2>Wallet action readiness</h2>
        <StatusPill label={kycGate.label} tone={kycGate.tone} />
      </div>
      <article className={styles.itemCard}>
        <div className={styles.itemHeader}>
          <h3>Identity gate</h3>
          <ShieldCheck size={18} aria-hidden />
        </div>
        <p className={styles.muted}>{kycGate.detail}</p>
        <p className={styles.muted}>{kycGate.actionDetail}</p>
      </article>
    </section>
  );

  return (
    <>
      {kycSection}
      {wallet ? metricsSection : walletSummarySection}
      {wallet ? walletSummarySection : metricsSection}
      <WalletTransferPanel
        enabled={kycGate.ready}
        onRefresh={onRefresh}
        walletLinked={Boolean(wallet)}
      />
      <WalletBurnPanel
        enabled={kycGate.ready}
        onRefresh={onRefresh}
        walletLinked={Boolean(wallet)}
      />

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Transfer history</h2>
          <StatusPill label={`${walletHistory.length} recent`} tone={walletHistory.length ? "good" : "neutral"} />
        </div>
        {walletHistory.length ? (
          <div className={styles.transferResultGrid}>
            {walletHistory.slice(0, 6).map((intent) => (
              <WalletDepositIntentHistoryCard key={intent.id} intent={intent} />
            ))}
          </div>
        ) : (
          <EmptyState
            detail="Persisted deposit intents appear here after you create wallet transfers."
            title="No transfer history yet"
          />
        )}
      </section>

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
