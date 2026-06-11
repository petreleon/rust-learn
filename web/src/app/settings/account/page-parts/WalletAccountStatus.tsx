"use client";

import { AlertCircle, CreditCard, Loader2 } from "lucide-react";
import Link from "next/link";
import { type WalletSummary } from "@/lib/learner";
import styles from "../page.module.css";
import { StatusLine } from "./StatusLine";
import { type RouteError } from "./RouteError";
import { type WalletLoadState } from "./WalletLoadState";

export function WalletAccountStatus({
  error,
  state,
  wallet,
}: {
  error: RouteError | null;
  state: WalletLoadState;
  wallet: WalletSummary | null;
}) {
  if (state === "loading") {
    return (
      <div className={styles.inlineStatus}>
        <Loader2 className={styles.spin} size={18} aria-hidden />
        <span>Checking wallet</span>
      </div>
    );
  }

  if (error) {
    return (
      <>
        <StatusLine icon={<AlertCircle size={16} aria-hidden />} label="Wallet unavailable" tone="warn" />
        <p className={styles.muted}>{error.message}</p>
        <Link className={styles.secondaryLink} href="/wallet">
          Open wallet
        </Link>
      </>
    );
  }

  if (!wallet) {
    return (
      <>
        <StatusLine icon={<CreditCard size={16} aria-hidden />} label="Wallet not linked" tone="warn" />
        <p className={styles.muted}>Link a RustLearn wallet before approved learner rewards can be credited.</p>
        <Link className={styles.primaryLink} href="/wallet">
          <CreditCard size={18} aria-hidden />
          Link wallet
        </Link>
      </>
    );
  }

  return (
    <>
      <StatusLine icon={<CreditCard size={16} aria-hidden />} label="Wallet linked" tone="good" />
      <div className={styles.walletValue}>
        <strong>{wallet.value}</strong>
        <span>{wallet.organization_id ? "Organization wallet" : "Personal wallet"}</span>
      </div>
      <p className={styles.muted}>Approved rewards can be credited to this wallet. Deposits and retirements are managed from the wallet route when available.</p>
      <Link className={styles.secondaryLink} href="/wallet">
        Open wallet
      </Link>
    </>
  );
}
