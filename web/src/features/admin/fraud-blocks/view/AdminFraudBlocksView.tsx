"use client";

import { ShieldAlert } from "lucide-react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { AdminDeniedState } from "@/components/admin-routes/AdminDeniedState";
import { GatedPanel } from "@/components/admin-routes/GatedPanel";
import { LoadingState } from "@/components/admin-routes/LoadingState";
import { SessionErrorState } from "@/components/admin-routes/SessionErrorState";
import { SignedOutState } from "@/components/admin-routes/SignedOutState";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { CreateFraudBlockPanel } from "../components/CreateFraudBlockPanel";
import { FraudBlockDetailPanel } from "../components/FraudBlockDetailPanel";
import { FraudBlockFiltersPanel } from "../components/FraudBlockFiltersPanel";
import { FraudBlockListPanel } from "../components/FraudBlockListPanel";
import { type AdminFraudBlocksRouteController } from "../route/useAdminFraudBlocksRoute";

function fraudBlockNotice(route: AdminFraudBlocksRouteController): ShellNotice | null {
  if (route.createState === "success") {
    return { message: "The fraud block was created.", title: "Block created", tone: "success" };
  }
  if (route.createState === "error" && route.createError) {
    return {
      message: route.createError.message,
      title: "Create failed",
      tone: route.createError.status === 403 ? "warn" : "error",
    };
  }
  if (route.revokeState === "success") {
    return { message: "The fraud block was revoked.", title: "Block revoked", tone: "success" };
  }
  if (route.revokeState === "error" && route.revokeError) {
    return {
      message: route.revokeError.message,
      title: "Revoke failed",
      tone: route.revokeError.status === 403 ? "warn" : "error",
    };
  }
  return null;
}

export function AdminFraudBlocksView({ route }: { route: AdminFraudBlocksRouteController }) {
  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Fraud blocks" },
      ]}
      description="Manage reward fraud blocks by scope, create new blocks, revoke existing ones, and inspect audit history."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={fraudBlockNotice(route)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : route.canView ? "Fraud access" : "Fraud gated"} />
          <StatusPill label={`${route.blocks?.total ?? 0} blocks`} />
          <StatusPill label={route.canCreate ? "Create enabled" : "Create gated"} tone={route.canCreate ? "good" : "neutral"} />
        </>
      }
      title="Fraud blocks"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed && !route.canView ? (
        <GatedPanel
          capability={route.fraudBlockCapability}
          icon={<ShieldAlert size={20} aria-hidden />}
          title="Fraud blocks unavailable"
        />
      ) : null}
      {route.session && route.allowed && route.canView ? <FraudBlockWorkspace route={route} /> : null}
    </ProductShell>
  );
}

function FraudBlockWorkspace({ route }: { route: AdminFraudBlocksRouteController }) {
  return (
    <div className={styles.stack}>
      <FraudBlockFiltersPanel
        filters={route.filters}
        onApply={route.applyFilters}
        onRefresh={route.loadBlocks}
        onReset={route.resetFilters}
        onUpdate={route.updateFilters}
        state={route.blocksState}
      />
      {route.blocksState === "success" && route.blocks ? (
        <div className={styles.twoColumnWide}>
          <FraudBlockListPanel
            blocks={route.blocks}
            error={route.blocksError}
            filters={route.filters}
            onPageOffset={route.setPageOffset}
            onRefresh={route.loadBlocks}
            onSelect={route.setSelectedBlockId}
            selectedBlockId={route.selectedBlockId}
            state={route.blocksState}
          />
          <FraudBlockSidePanel route={route} />
        </div>
      ) : (
        <FraudBlockListPanel
          blocks={route.blocks}
          error={route.blocksError}
          filters={route.filters}
          onPageOffset={route.setPageOffset}
          onRefresh={route.loadBlocks}
          onSelect={route.setSelectedBlockId}
          selectedBlockId={route.selectedBlockId}
          state={route.blocksState}
        />
      )}
    </div>
  );
}

function FraudBlockSidePanel({ route }: { route: AdminFraudBlocksRouteController }) {
  const block = route.selectedBlock;

  if (block) {
    return (
      <FraudBlockDetailPanel
        auditError={route.auditError}
        auditEvents={route.auditEvents}
        auditState={route.auditState}
        block={block}
        canRevoke={route.canRevoke}
        onRefreshAudit={() => void route.loadAudit(block.id)}
        onRevoke={route.handleRevoke}
        revokeState={route.revokeState}
      />
    );
  }
  if (route.canCreate) {
    return (
      <CreateFraudBlockPanel
        draft={route.createDraft}
        error={route.createError}
        onDraftChange={route.updateCreateDraft}
        onSubmit={route.handleCreate}
        state={route.createState}
      />
    );
  }

  return (
    <GatedPanel
      capability={route.fraudBlockCapability}
      icon={<ShieldAlert size={20} aria-hidden />}
      title="Fraud block creation unavailable"
    />
  );
}
