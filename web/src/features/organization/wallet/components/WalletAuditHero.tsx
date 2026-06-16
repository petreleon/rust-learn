"use client";

import { AlertTriangle, ArrowLeft, CheckCircle2, CreditCard, Loader2, RefreshCw } from "lucide-react";
import Link from "next/link";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { type RouteError } from "@/shared/route-state/RouteError";
import { StatusPill } from "@/features/organization/shared/route-kit/StatusPill";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { type WalletLinkState } from "../model/WalletLinkState";

export function WalletAuditHero({
  canManageBudget,
  canManageWallets,
  linkError,
  linkState,
  onLinkWallet,
  onRefresh,
  organization,
}: {
  canManageBudget: boolean;
  canManageWallets: boolean;
  linkError: RouteError | null;
  linkState: WalletLinkState;
  onLinkWallet: () => void;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  return (
    <section className={styles.workspaceHero}>
      <div className={styles.workspaceTitleBlock}>
        <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
          <ArrowLeft size={17} aria-hidden />
          {organization.name}
        </Link>
        <p className={styles.eyebrow}>Wallet and budget</p>
        <h2>Organization wallet audit</h2>
        <p className={styles.muted}>
          Wallet audit rows combine internal ledger entries, token payout references, source organization reward records, and
          compensation adjustments from the current backend contract.
        </p>
      </div>
      <div className={styles.actionRow}>
        <button className={styles.secondaryButton} onClick={onRefresh} type="button">
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
        {canManageWallets ? (
          <button className={styles.secondaryButton} disabled={linkState === "linking"} onClick={onLinkWallet} type="button">
            {linkState === "linking" ? <Loader2 className={styles.spin} size={17} aria-hidden /> : <CreditCard size={17} aria-hidden />}
            Relink wallet
          </button>
        ) : null}
      </div>
      <div className={styles.permissionRow}>
        {canManageWallets ? <span className={styles.permissionChip}>Can manage wallets</span> : null}
        {canManageBudget ? <span className={styles.permissionChip}>Can manage reward budget</span> : null}
        {!canManageWallets && !canManageBudget ? <span className={styles.permissionChip}>Read-only wallet audit</span> : null}
        {linkState === "success" ? <StatusPill icon={<CheckCircle2 size={16} aria-hidden />} label="Wallet linked" tone="good" /> : null}
      </div>
      {linkError ? (
        <section className={styles.inlineError} role="status">
          <AlertTriangle size={17} aria-hidden />
          <span>{linkError.message}</span>
        </section>
      ) : null}
    </section>
  );
}
