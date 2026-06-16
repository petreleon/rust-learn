"use client";

import { Flame } from "lucide-react";
import { StatusPill } from "../../components/StatusPill";
import { WalletBurnForm } from "../components/WalletBurnForm";
import { WalletBurnResultCard } from "../components/WalletBurnResultCard";
import { type WalletBurnDraft, type WalletBurnResult } from "../model/walletBurnTypes";
import styles from "../../learner-workspace.module.css";

export function WalletBurnView({
  burns,
  draft,
  enabled,
  error,
  loading,
  onSubmit,
  onUpdateDraft,
  submitting,
  walletLinked,
}: {
  burns: WalletBurnResult[];
  draft: WalletBurnDraft;
  enabled: boolean;
  error: string | null;
  loading: boolean;
  submitting: boolean;
  walletLinked: boolean;
  onSubmit: () => void;
  onUpdateDraft: (patch: Partial<WalletBurnDraft>) => void;
}) {
  const ready = enabled && walletLinked;

  return (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <div>
          <h2>Burn tokens</h2>
          <p className={styles.muted}>Burn from your wallet balance or record a confirmed MetaMask burn.</p>
        </div>
        <StatusPill label={burnStatusLabel(enabled, walletLinked, loading)} tone={ready ? "good" : "warn"} />
      </div>
      {error ? <p className={styles.errorPanel}>{error}</p> : null}
      <WalletBurnForm
        draft={draft}
        enabled={ready}
        isSubmitting={submitting}
        onSubmit={onSubmit}
        onUpdate={onUpdateDraft}
      />
      <div className={styles.sectionHeader}>
        <h3>Recent burns</h3>
        <Flame size={18} aria-hidden />
      </div>
      {burns.length ? (
        <div className={styles.transferResultGrid}>
          {burns.slice(0, 6).map((burn) => (
            <WalletBurnResultCard burn={burn} key={burn.id} />
          ))}
        </div>
      ) : (
        <p className={styles.muted}>Created token burns will appear here during this session.</p>
      )}
    </section>
  );
}

function burnStatusLabel(enabled: boolean, walletLinked: boolean, loading: boolean) {
  if (!walletLinked) return "Wallet required";
  if (!enabled) return "KYC required";
  return loading ? "Loading" : "Available";
}
