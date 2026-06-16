"use client";

import { ShieldCheck } from "lucide-react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import styles from "@/components/admin-routes.module.css";
import { AdminDeniedState } from "@/components/admin-routes/AdminDeniedState";
import { GatedPanel } from "@/components/admin-routes/GatedPanel";
import { LoadingState } from "@/components/admin-routes/LoadingState";
import { SessionErrorState } from "@/components/admin-routes/SessionErrorState";
import { SignedOutState } from "@/components/admin-routes/SignedOutState";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { CreateDelegationPanel } from "../components/CreateDelegationPanel";
import { DelegationDetailPanel } from "../components/DelegationDetailPanel";
import { DelegationListPanel } from "../components/DelegationListPanel";
import { type AdminDelegationsRouteController } from "../route/useAdminDelegationsRoute";

function delegationNotice(route: AdminDelegationsRouteController): ShellNotice | null {
  if (route.createState === "success") {
    return { message: "The delegation was created.", title: "Delegation created", tone: "success" };
  }
  if (route.createState === "error" && route.createError) {
    return {
      message: route.createError.message,
      title: "Create failed",
      tone: route.createError.status === 403 ? "warn" : "error",
    };
  }
  if (route.revokeState === "success") {
    return { message: "The delegation was revoked.", title: "Delegation revoked", tone: "success" };
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

export function AdminDelegationsView({ route }: { route: AdminDelegationsRouteController }) {
  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Delegations" },
      ]}
      description="Manage delegated permissions with scope, grantee, expiration, and revocation audit."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={delegationNotice(route)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : route.canView ? "Delegation access" : "Delegation gated"} />
          <StatusPill label={`${route.delegations.length} delegations`} />
          <StatusPill label={route.canGrant ? "Grant enabled" : "Grant gated"} tone={route.canGrant ? "good" : "neutral"} />
        </>
      }
      title="Delegated permissions"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed && !route.canView ? (
        <GatedPanel capability={route.delegationCapability} icon={<ShieldCheck size={20} aria-hidden />} title="Delegated permissions unavailable" />
      ) : null}
      {route.session && route.allowed && route.canView ? (
        <div className={styles.twoColumnWide}>
          <DelegationListPanel
            delegations={route.delegations}
            error={route.delegationError}
            onRefresh={route.loadDelegations}
            onSelect={route.setSelectedDelegation}
            selectedDelegation={route.selectedDelegation}
            state={route.delegationState}
          />
          <div>
            {route.selectedDelegation ? (
              <DelegationDetailPanel
                canRevoke={route.canRevoke}
                delegation={route.selectedDelegation}
                onRevoke={route.handleRevoke}
                onRevokeReasonChange={route.setRevokeReason}
                revokeError={route.revokeError}
                revokeReason={route.revokeReason}
                revokeState={route.revokeState}
              />
            ) : route.canGrant ? (
              <CreateDelegationPanel
                draft={route.createDraft}
                error={route.createError}
                onDraftChange={route.updateCreateDraft}
                onSubmit={route.handleCreate}
                state={route.createState}
              />
            ) : (
              <GatedPanel capability={route.delegationCapability} icon={<ShieldCheck size={20} aria-hidden />} title="Delegation grant unavailable" />
            )}
          </div>
        </div>
      ) : null}
    </ProductShell>
  );
}
