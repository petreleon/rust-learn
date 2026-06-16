"use client";

import { type WalletBurnResult } from "../model/walletBurnTypes";
import { StatusPill } from "../../components/StatusPill";
import { humanize } from "../../components/humanize";
import styles from "../../learner-workspace.module.css";

export function WalletBurnResultCard({ burn }: { burn: WalletBurnResult }) {
  return (
    <article className={styles.transferResultCard}>
      <div className={styles.itemHeader}>
        <h4>{burn.amount} LearnToken burned</h4>
        <StatusPill label={humanize(burn.status)} tone={burn.metamask_required ? "warn" : "good"} />
      </div>
      <p className={styles.muted}>{burnActionCopy(burn)}</p>
      <div className={styles.metaRow}>
        <span>{humanize(burn.source)}</span>
        <span>{humanize(burn.fee_path)}</span>
        <span>{burn.fee_amount} fee</span>
      </div>
      <div className={styles.detailList}>
        <span>{humanize(burn.wallet_action)}</span>
        <span>{burn.leaderboard_visible ? "Leaderboard visible" : "Hidden from leaderboard"}</span>
      </div>
    </article>
  );
}

function burnActionCopy(burn: WalletBurnResult) {
  if (burn.status === "deposit_pending") return "Platform-mediated burn is waiting for the deposit step.";
  if (burn.status === "failed") return "Burn execution failed and needs review.";
  if (burn.status === "needs_reconciliation") return "Burn evidence needs reconciliation.";
  return "Burn evidence is recorded for wallet history and leaderboard indexing.";
}
