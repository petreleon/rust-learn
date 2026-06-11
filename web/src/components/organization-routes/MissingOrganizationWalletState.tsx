"use client";

import { AlertTriangle, CreditCard, Loader2, RefreshCw } from "lucide-react";
import Link from "next/link";
import { type OrganizationWorkspaceItem } from "@/lib/organization";
import styles from "../organization-routes.module.css";
import { type RouteError } from "./RouteError";
import { type WalletLinkState } from "./WalletLinkState";

export function MissingOrganizationWalletState({
  canManageWallets,
  linkError,
  linkState,
  onLinkWallet,
  onRefresh,
  organization,
}: {
  canManageWallets: boolean;
  linkError: RouteError | null;
  linkState: WalletLinkState;
  onLinkWallet: () => void;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <CreditCard size={20} aria-hidden />
        <h2>Wallet not linked</h2>
      </div>
      <p className={styles.muted}>
        {organization.name} does not have an organization wallet attached yet. Link the wallet
        before relying on balance, reward-credit, or token reconciliation views.
      </p>
      <div className={styles.actionRow}>
        {canManageWallets ? (
          <button
            className={styles.primaryButton}
            disabled={linkState === "linking"}
            onClick={onLinkWallet}
            type="button"
          >
            {linkState === "linking" ? (
              <Loader2 className={styles.spin} size={17} aria-hidden />
            ) : (
              <CreditCard size={17} aria-hidden />
            )}
            Link organization wallet
          </button>
        ) : (
          <span className={styles.permissionChip}>Wallet manager required</span>
        )}
        <button className={styles.secondaryButton} onClick={onRefresh} type="button">
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </div>
      {!canManageWallets ? (
        <div className={styles.missingList}>
          <strong>Missing scoped permission</strong>
          <span>MANAGE_ORG_WALLETS</span>
        </div>
      ) : null}
      {linkError ? (
        <section className={styles.inlineError} role="status">
          <AlertTriangle size={17} aria-hidden />
          <span>{linkError.message}</span>
        </section>
      ) : null}
    </section>
  );
}
