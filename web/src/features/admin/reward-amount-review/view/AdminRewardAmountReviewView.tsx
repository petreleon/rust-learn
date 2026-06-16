"use client";

import { Landmark } from "lucide-react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { AdminDeniedState } from "@/components/admin-routes/AdminDeniedState";
import { GatedPanel } from "@/components/admin-routes/GatedPanel";
import { LoadingState } from "@/components/admin-routes/LoadingState";
import { SessionErrorState } from "@/components/admin-routes/SessionErrorState";
import { SignedOutState } from "@/components/admin-routes/SignedOutState";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { RewardCandidateDetailPanel } from "../components/RewardCandidateDetailPanel";
import { RewardCandidateFiltersPanel } from "../components/RewardCandidateFiltersPanel";
import { RewardCandidateListPanel } from "../components/RewardCandidateListPanel";
import { RewardCandidateSummaryPanel } from "../components/RewardCandidateSummaryPanel";
import { type AdminRewardAmountReviewRouteController } from "../route/useAdminRewardAmountReviewRoute";

function rewardAmountNotice(route: AdminRewardAmountReviewRouteController): ShellNotice | null {
  if (route.decisionState === "success") {
    return {
      message: "The candidate was refreshed with the latest decision state.",
      title: "Decision saved",
      tone: "success",
    };
  }
  if (route.decisionState === "error" && route.decisionError) {
    return {
      message: route.decisionError.message,
      title: route.decisionError.status === 409 ? "Candidate changed" : "Decision failed",
      tone: route.decisionError.status === 403 || route.decisionError.status === 409 ? "warn" : "error",
    };
  }
  return null;
}

export function AdminRewardAmountReviewView({
  route,
}: {
  route: AdminRewardAmountReviewRouteController;
}) {
  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Reward amount review" },
      ]}
      description="Review teacher-approved reward candidates, set approved amounts, and inspect audit history."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={rewardAmountNotice(route)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : route.canView ? "Review access" : "Review gated"} />
          <StatusPill label={`${route.candidates?.total ?? 0} candidates`} />
          <StatusPill label={route.canApprove ? "Approve enabled" : "Approve gated"} tone={route.canApprove ? "good" : "neutral"} />
        </>
      }
      title="Reward amount review"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed && !route.canView ? (
        <GatedPanel
          capability={route.rewardAmountCapability}
          icon={<Landmark size={20} aria-hidden />}
          title="Reward amount review unavailable"
        />
      ) : null}
      {route.session && route.allowed && route.canView ? <RewardAmountWorkspace route={route} /> : null}
    </ProductShell>
  );
}

function RewardAmountWorkspace({ route }: { route: AdminRewardAmountReviewRouteController }) {
  return (
    <div className={styles.stack}>
      <RewardCandidateSummaryPanel candidates={route.candidates} state={route.candidateState} />
      <RewardCandidateFiltersPanel
        filters={route.filters}
        onApply={route.applyFilters}
        onRefresh={route.loadCandidates}
        onReset={route.resetFilters}
        onUpdate={route.updateFilters}
        state={route.candidateState}
      />
      {route.candidateState === "success" && route.candidates ? (
        <div className={styles.twoColumnWide}>
          <RewardCandidateListPanel
            candidates={route.candidates}
            error={route.candidateError}
            filters={route.filters}
            onPageOffset={route.setPageOffset}
            onRefresh={route.loadCandidates}
            onSelect={route.setSelectedCandidateId}
            selectedCandidateId={route.selectedCandidateId}
            state={route.candidateState}
          />
          <RewardAmountSidePanel route={route} />
        </div>
      ) : (
        <RewardCandidateListPanel
          candidates={route.candidates}
          error={route.candidateError}
          filters={route.filters}
          onPageOffset={route.setPageOffset}
          onRefresh={route.loadCandidates}
          onSelect={route.setSelectedCandidateId}
          selectedCandidateId={route.selectedCandidateId}
          state={route.candidateState}
        />
      )}
    </div>
  );
}

function RewardAmountSidePanel({ route }: { route: AdminRewardAmountReviewRouteController }) {
  const candidate = route.selectedCandidate;

  return (
    <RewardCandidateDetailPanel
      auditError={route.auditError}
      auditEvents={route.auditEvents}
      auditState={route.auditState}
      canApprove={route.canApprove}
      candidate={candidate}
      decisionDraft={route.decisionDraft}
      decisionError={route.decisionError}
      decisionState={route.decisionState}
      onDecisionDraftChange={route.updateDecisionDraft}
      onRefreshAudit={() => (candidate ? void route.loadAudit(candidate.id) : undefined)}
      onSubmitDecision={route.handleDecision}
    />
  );
}
