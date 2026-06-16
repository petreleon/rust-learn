"use client";

import { type WalletTransferResult } from "../model/walletTransferTypes";
import {
  walletTransferActionCopy,
  walletTransferDelta,
  walletTransferStatus,
  walletTransferTitle,
} from "../model/walletTransferCopy";
import { StatusPill } from "../../components/StatusPill";
import { humanize } from "../../components/humanize";
import styles from "../../learner-workspace.module.css";

export function WalletTransferResultCard({ result }: { result: WalletTransferResult }) {
  return (
    <article className={styles.transferResultCard}>
      <div className={styles.itemHeader}>
        <h4>{walletTransferTitle(result)}</h4>
        <StatusPill label={humanize(walletTransferStatus(result))} tone={result.metamask_required ? "warn" : "good"} />
      </div>
      <p className={styles.muted}>{walletTransferActionCopy(result)}</p>
      <div className={styles.metaRow}>
        <span>{result.amount} amount</span>
        <span>{result.tax_amount} tax</span>
        <span>{walletTransferDelta(result)} wallet delta</span>
      </div>
      <div className={styles.detailList}>
        <span>{humanize(result.gas_payer)} gas payer</span>
        <span>{humanize(result.wallet_provider)} wallet provider</span>
        <span>{humanize(result.wallet_action)}</span>
        {result.operation === "deposit" ? <span>Receiver {result.platform_address}</span> : null}
      </div>
    </article>
  );
}
