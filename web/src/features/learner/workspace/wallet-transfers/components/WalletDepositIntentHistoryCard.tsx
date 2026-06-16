"use client";

import { type WalletDepositIntentAudit } from "@/lib/learner";
import { StatusPill } from "../../components/StatusPill";
import { formatDate } from "../../components/formatDate";
import { humanize } from "../../components/humanize";
import styles from "../../learner-workspace.module.css";
import {
  walletDepositIntentNextStep,
  walletDepositIntentStatusLabel,
  walletDepositIntentTone,
} from "../model/walletDepositIntentHistory";

export function WalletDepositIntentHistoryCard({ intent }: { intent: WalletDepositIntentAudit }) {
  return (
    <article className={styles.transferResultCard}>
      <div className={styles.itemHeader}>
        <h4>Deposit intent #{intent.id}</h4>
        <StatusPill label={walletDepositIntentStatusLabel(intent.status)} tone={walletDepositIntentTone(intent.status)} />
      </div>
      <p className={styles.muted}>{walletDepositIntentNextStep(intent)}</p>
      <div className={styles.metaRow}>
        <span>{intent.amount} amount</span>
        <span>{intent.tax_amount} tax</span>
        <span>{humanize(intent.gas_payer)} gas payer</span>
      </div>
      <div className={styles.detailList}>
        <span>Updated {formatDate(intent.updated_at)}</span>
        {intent.credited_at ? <span>Credited {formatDate(intent.credited_at)}</span> : null}
        {intent.transaction_hash ? <span>Token tx {intent.transaction_hash.slice(0, 12)}</span> : null}
        <span>{humanize(intent.wallet_action)}</span>
        <span>Receiver {intent.platform_address}</span>
      </div>
    </article>
  );
}
