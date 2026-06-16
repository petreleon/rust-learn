"use client";

import { ListChecks } from "lucide-react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { AdminDeniedState } from "@/features/admin/shared/route-kit/AdminDeniedState";
import { GatedPanel } from "@/features/admin/shared/route-kit/GatedPanel";
import { LoadingState } from "@/features/admin/shared/route-kit/LoadingState";
import { SessionErrorState } from "@/features/admin/shared/route-kit/SessionErrorState";
import { SignedOutState } from "@/features/admin/shared/route-kit/SignedOutState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { RewardPolicyCreatePanel } from "../components/RewardPolicyCreatePanel";
import { RewardPolicyFiltersPanel } from "../components/RewardPolicyFiltersPanel";
import { RewardPolicyListPanel } from "../components/RewardPolicyListPanel";
import { type AdminRewardPoliciesRouteController } from "../route/useAdminRewardPoliciesRoute";

function rewardPolicyNotice(route: AdminRewardPoliciesRouteController): ShellNotice | null {
  if (route.createState === "success" && route.createdPolicy) {
    return {
      message: `Policy #${route.createdPolicy.id} is ready for ${route.createdPolicy.scope_type} scope.`,
      title: "Policy created",
      tone: "success",
    };
  }
  if (route.createState === "error" && route.createError) {
    return {
      message: route.createError.message,
      title: "Create failed",
      tone: route.createError.status === 403 ? "warn" : "error",
    };
  }
  return null;
}

export function AdminRewardPoliciesView({ route }: { route: AdminRewardPoliciesRouteController }) {
  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[{ label: "Admin", href: "/admin" }, { label: "Reward policies" }]}
      description="Create and review policy versions by scope, event type, payment strategy, and active state."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={rewardPolicyNotice(route)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : route.canManage ? "Policy access" : "Policy gated"} />
          <StatusPill label={`${route.policies.length} loaded`} />
          <StatusPill label={route.canManage ? "Create enabled" : "Create gated"} tone={route.canManage ? "good" : "neutral"} />
        </>
      }
      title="Reward policies"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed && !route.canManage ? (
        <GatedPanel capability={route.rewardPolicyCapability} icon={<ListChecks size={20} aria-hidden />} title="Reward policies unavailable" />
      ) : null}
      {route.session && route.allowed && route.canManage ? <RewardPolicyWorkspace route={route} /> : null}
    </ProductShell>
  );
}

function RewardPolicyWorkspace({ route }: { route: AdminRewardPoliciesRouteController }) {
  return (
    <div className={styles.stack}>
      <RewardPolicyFiltersPanel
        filters={route.filters}
        onApply={route.applyFilters}
        onRefresh={route.loadPolicies}
        onReset={route.resetFilters}
        onUpdate={route.updateFilters}
        state={route.policiesState}
      />
      <div className={styles.twoColumnWide}>
        <RewardPolicyListPanel
          error={route.policiesError}
          filters={route.filters}
          hasNextPage={route.hasNextPage}
          onPageOffset={route.setPageOffset}
          onRefresh={route.loadPolicies}
          policies={route.policies}
          state={route.policiesState}
        />
        <RewardPolicyCreatePanel
          draft={route.draft}
          error={route.createError}
          onDraftChange={route.updateDraft}
          onSubmit={route.createPolicy}
          state={route.createState}
        />
      </div>
    </div>
  );
}
