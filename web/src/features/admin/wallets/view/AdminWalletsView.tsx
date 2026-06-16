"use client";

import { WalletCards } from "lucide-react";
import { ProductShell } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { AdminDeniedState } from "@/features/admin/shared/route-kit/AdminDeniedState";
import { GatedPanel } from "@/features/admin/shared/route-kit/GatedPanel";
import { LoadingState } from "@/features/admin/shared/route-kit/LoadingState";
import { SessionErrorState } from "@/features/admin/shared/route-kit/SessionErrorState";
import { SignedOutState } from "@/features/admin/shared/route-kit/SignedOutState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { WalletCreditsCsvPanel } from "../components/WalletCreditsCsvPanel";
import { WalletReconciliationPanel } from "../components/WalletReconciliationPanel";
import { WalletSummaryPanel } from "../components/WalletSummaryPanel";
import { WalletTokenTaxAuditPanel } from "../components/WalletTokenTaxAuditPanel";
import { WalletTokenTaxPanel } from "../components/WalletTokenTaxPanel";
import { type AdminWalletsRouteController } from "../route/useAdminWalletsRoute";

export function AdminWalletsView({ route }: { route: AdminWalletsRouteController }) {
  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Wallets" },
      ]}
      description="Platform wallet summary and transaction audit."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill
            label={
              route.loadState === "loading"
                ? "Resolving session"
                : route.canView || route.canManageTaxes
                  ? "Wallet access"
                  : "Wallet gated"
            }
          />
          <StatusPill
            label={route.summary ? `${route.summary.total_wallets} wallets` : route.tokenTaxSettings ? "Token taxes" : "Loading"}
          />
        </>
      }
      title="Wallet audit"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed && !route.canView && !route.canManageTaxes ? (
        <GatedPanel capability={route.walletCapability} icon={<WalletCards size={20} aria-hidden />} title="Wallet audit unavailable" />
      ) : null}
      {route.session && route.allowed && (route.canView || route.canManageTaxes) ? (
        <div className={styles.stack}>
          {route.canManageTaxes ? (
            <>
              <WalletTokenTaxPanel
                canSetDeposit={route.canSetDeposit}
                canSetRetire={route.canSetRetire}
                draft={route.tokenTaxDraft}
                error={route.tokenTaxError}
                savingOperation={route.tokenTaxSavingOperation}
                settings={route.tokenTaxSettings}
                state={route.tokenTaxState}
                onChange={route.changeTokenTax}
                onRetry={route.loadTokenTaxes}
                onSave={route.saveTokenTax}
              />
              {route.tokenTaxSettings ? (
                <WalletTokenTaxAuditPanel
                  events={route.tokenTaxAuditEvents}
                  settings={route.tokenTaxSettings}
                />
              ) : null}
            </>
          ) : null}
          {route.canView ? (
            <>
              <WalletSummaryPanel
                error={route.summaryError}
                onRetry={route.loadWalletAudit}
                state={route.summaryState}
                summary={route.summary}
              />
              <WalletCreditsCsvPanel
                canExport={route.canExport}
                csvError={route.csvError}
                csvState={route.csvState}
                onDownload={route.handleDownload}
              />
              <WalletReconciliationPanel
                error={route.reconciliationError}
                onRetry={route.loadWalletAudit}
                reconciliation={route.reconciliation}
                state={route.reconciliationState}
              />
            </>
          ) : null}
        </div>
      ) : null}
    </ProductShell>
  );
}
