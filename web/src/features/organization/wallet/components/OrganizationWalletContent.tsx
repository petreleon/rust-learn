"use client";

import { Loader2 } from "lucide-react";
import { type OrganizationWalletAudit } from "@/lib/organization/OrganizationWalletAudit";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { type RouteError } from "@/shared/route-state/RouteError";
import { MissingOrganizationWalletState } from "@/components/organization-routes/MissingOrganizationWalletState";
import { WalletErrorState } from "@/components/organization-routes/WalletErrorState";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { type WalletLinkState } from "../model/WalletLinkState";
import { type WalletLoadState } from "../model/WalletLoadState";
import { walletAuditSummary } from "../model/walletAuditSummary";
import { CompensationAdjustmentsPanel } from "./CompensationAdjustmentsPanel";
import { RewardCreditAuditPanel } from "./RewardCreditAuditPanel";
import { WalletAuditHero } from "./WalletAuditHero";
import { WalletBudgetCoveragePanels } from "./WalletBudgetCoveragePanels";
import { WalletSummaryGrid } from "./WalletSummaryGrid";
import { WalletTransactionPanels } from "./WalletTransactionPanels";

type Props = {
  audit: OrganizationWalletAudit | null;
  canManageBudget: boolean;
  canManageWallets: boolean;
  linkError: RouteError | null;
  linkState: WalletLinkState;
  loadState: WalletLoadState;
  onLinkWallet: () => void;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
  walletError: RouteError | null;
};

export function OrganizationWalletContent(props: Props) {
  if (props.loadState === "loading" || props.loadState === "idle") {
    return <WalletLoadingState />;
  }

  if (props.loadState === "error") {
    return <WalletErrorState error={props.walletError} onRetry={props.onRefresh} />;
  }

  if (props.loadState === "missing") {
    return (
      <MissingOrganizationWalletState
        canManageWallets={props.canManageWallets}
        linkError={props.linkError}
        linkState={props.linkState}
        onLinkWallet={props.onLinkWallet}
        onRefresh={props.onRefresh}
        organization={props.organization}
      />
    );
  }

  if (!props.audit) {
    return null;
  }

  const summary = walletAuditSummary(props.audit);

  return (
    <>
      <WalletAuditHero
        canManageBudget={props.canManageBudget}
        canManageWallets={props.canManageWallets}
        linkError={props.linkError}
        linkState={props.linkState}
        onLinkWallet={props.onLinkWallet}
        onRefresh={props.onRefresh}
        organization={props.organization}
      />
      <WalletSummaryGrid audit={props.audit} attentionCount={summary.attentionRecords.length} />
      <WalletBudgetCoveragePanels audit={props.audit} />
      <RewardCreditAuditPanel audit={props.audit} />
      <WalletTransactionPanels audit={props.audit} />
      <CompensationAdjustmentsPanel records={props.audit.compensation_records} />
    </>
  );
}

function WalletLoadingState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Loading wallet audit</h2>
      </div>
      <div className={styles.skeletonGrid} aria-hidden>
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
      </div>
    </section>
  );
}
