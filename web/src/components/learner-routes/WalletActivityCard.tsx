"use client";

import { type RewardHistoryEntry } from "@/lib/learner";
import styles from "../learner-routes.module.css";
import { StatusPill } from "./StatusPill";
import { formatDate } from "./formatDate";
import { humanRewardStatus } from "./humanRewardStatus";
import { humanize } from "./humanize";
import { rewardNextStep } from "./rewardNextStep";
import { rewardTone } from "./rewardTone";

export function WalletActivityCard({ reward }: { reward: RewardHistoryEntry }) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{reward.course_title}</h3>
        <StatusPill label={reward.wallet_credit ? "Wallet credited" : humanRewardStatus(reward.status)} tone={rewardTone(reward.status)} />
      </div>
      <p className={styles.muted}>{rewardNextStep(reward)}</p>
      <div className={styles.metaRow}>
        <span>{humanize(reward.event_type)}</span>
        <span>{reward.approved_amount ? `${reward.approved_amount} approved` : "Amount pending"}</span>
        <span>{reward.wallet_credit ? `Credited ${reward.wallet_credit.amount}` : "Wallet pending"}</span>
      </div>
      <div className={styles.detailList}>
        <span>Updated {formatDate(reward.updated_at)}</span>
        {reward.wallet_credit ? <span>Credited {formatDate(reward.wallet_credit.credited_at)}</span> : null}
        {reward.token_transaction?.transaction_hash ? (
          <span>Token tx {reward.token_transaction.transaction_hash.slice(0, 12)}</span>
        ) : null}
      </div>
    </article>
  );
}
