"use client";

import { WalletCards } from "lucide-react";
import { ProductShell } from "@/components/product-shell";
import styles from "@/components/admin-routes.module.css";
import { AdminDeniedState } from "@/components/admin-routes/AdminDeniedState";
import { GatedPanel } from "@/components/admin-routes/GatedPanel";
import { LoadingState } from "@/components/admin-routes/LoadingState";
import { SessionErrorState } from "@/components/admin-routes/SessionErrorState";
import { SignedOutState } from "@/components/admin-routes/SignedOutState";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { WalletCreditsCsvPanel } from "../components/WalletCreditsCsvPanel";
import { WalletReconciliationPanel } from "../components/WalletReconciliationPanel";
import { WalletSummaryPanel } from "../components/WalletSummaryPanel";
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
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : route.canView ? "Wallet access" : "Wallet gated"} />
          <StatusPill label={route.summary ? `${route.summary.total_wallets} wallets` : "Loading"} />
        </>
      }
      title="Wallet audit"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed && !route.canView ? (
        <GatedPanel capability={route.walletCapability} icon={<WalletCards size={20} aria-hidden />} title="Wallet audit unavailable" />
      ) : null}
      {route.session && route.allowed && route.canView ? (
        <div className={styles.stack}>
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
        </div>
      ) : null}
    </ProductShell>
  );
}
