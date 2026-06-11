"use client";

import Link from "next/link";
import { type WalletSummary } from "@/lib/learner";
import styles from "../learner-routes.module.css";
import { StatusPill } from "./StatusPill";

export function DashboardWalletPanel({ wallet }: { wallet: WalletSummary | null }) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>Wallet readiness</h3>
        <StatusPill label={wallet ? "Linked" : "Unlinked"} tone={wallet ? "good" : "warn"} />
      </div>
      {wallet ? (
        <>
          <p className={styles.muted}>Approved rewards can be credited to this wallet.</p>
          <strong className={styles.walletValue}>{wallet.value}</strong>
          <div className={styles.metaRow}>
            <span>{wallet.owner_type}</span>
            <span>{wallet.organization_id ? "Organization wallet" : "Personal wallet"}</span>
          </div>
        </>
      ) : (
        <p className={styles.muted}>Link your RustLearn wallet before approved rewards can be credited.</p>
      )}
      <div className={styles.actionRow}>
        <Link className={wallet ? styles.secondaryLink : styles.primaryLink} href="/wallet">
          {wallet ? "Open wallet" : "Link wallet"}
        </Link>
      </div>
    </article>
  );
}
