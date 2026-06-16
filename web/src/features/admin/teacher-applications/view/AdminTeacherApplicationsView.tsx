"use client";

import { UserCheck } from "lucide-react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { AdminDeniedState } from "@/features/admin/shared/route-kit/AdminDeniedState";
import { GatedPanel } from "@/features/admin/shared/route-kit/GatedPanel";
import { LoadingState } from "@/features/admin/shared/route-kit/LoadingState";
import { SessionErrorState } from "@/features/admin/shared/route-kit/SessionErrorState";
import { SignedOutState } from "@/features/admin/shared/route-kit/SignedOutState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { TeacherApplicationDetailPanel } from "../components/TeacherApplicationDetailPanel";
import { TeacherApplicationFilterPanel } from "../components/TeacherApplicationFilterPanel";
import { TeacherApplicationQueuePanel } from "../components/TeacherApplicationQueuePanel";
import { TeacherApplicationSummaryPanel } from "../components/TeacherApplicationSummaryPanel";
import { type AdminTeacherApplicationsRouteController } from "../route/useAdminTeacherApplicationsRoute";

function teacherApplicationNotice(route: AdminTeacherApplicationsRouteController): ShellNotice | null {
  if (route.decisionState === "success") {
    return {
      message: "The application was refreshed with the latest decision state.",
      title: "Decision saved",
      tone: "success",
    };
  }
  if (route.decisionState === "error" && route.decisionError) {
    return {
      message: route.decisionError.message,
      title: route.decisionError.status === 409 ? "Application changed" : "Decision failed",
      tone: route.decisionError.status === 403 || route.decisionError.status === 409 ? "warn" : "error",
    };
  }
  return null;
}

export function AdminTeacherApplicationsView({
  route,
}: {
  route: AdminTeacherApplicationsRouteController;
}) {
  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Teacher applications" },
      ]}
      description="Review submitted teacher applications with applicant context, sponsor scope, audit history, and permission-gated decisions."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={teacherApplicationNotice(route)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : route.canReview ? "Review access" : "Review gated"} />
          <StatusPill label={`${route.applications?.summary.submitted ?? 0} submitted`} tone={(route.applications?.summary.submitted ?? 0) ? "warn" : "neutral"} />
          <StatusPill label={route.canApprove ? "Approve enabled" : "Approve gated"} tone={route.canApprove ? "good" : "neutral"} />
        </>
      }
      title="Teacher application review"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed && !route.canReview ? (
        <GatedPanel
          capability={route.teacherApplicationCapability}
          icon={<UserCheck size={20} aria-hidden />}
          title="Teacher application review unavailable"
        />
      ) : null}
      {route.session && route.allowed && route.canReview ? <TeacherApplicationWorkspace route={route} /> : null}
    </ProductShell>
  );
}

function TeacherApplicationWorkspace({ route }: { route: AdminTeacherApplicationsRouteController }) {
  return (
    <div className={styles.stack}>
      <TeacherApplicationSummaryPanel response={route.applications} state={route.applicationState} />
      <TeacherApplicationFilterPanel
        filters={route.filters}
        onApply={route.applyFilters}
        onRefresh={route.loadApplications}
        onReset={route.resetFilters}
        onUpdate={route.updateFilters}
        state={route.applicationState}
      />
      {route.applicationState === "success" && route.applications ? (
        <div className={styles.twoColumnWide}>
          <TeacherApplicationQueuePanel
            applications={route.applications}
            error={route.applicationError}
            filters={route.filters}
            onPageOffset={route.setPageOffset}
            onRefresh={route.loadApplications}
            onSelect={route.setSelectedApplicationId}
            selectedApplicationId={route.selectedApplicationId}
            state={route.applicationState}
          />
          <TeacherApplicationSidePanel route={route} />
        </div>
      ) : (
        <TeacherApplicationQueuePanel
          applications={route.applications}
          error={route.applicationError}
          filters={route.filters}
          onPageOffset={route.setPageOffset}
          onRefresh={route.loadApplications}
          onSelect={route.setSelectedApplicationId}
          selectedApplicationId={route.selectedApplicationId}
          state={route.applicationState}
        />
      )}
    </div>
  );
}

function TeacherApplicationSidePanel({ route }: { route: AdminTeacherApplicationsRouteController }) {
  const application = route.selectedApplication;

  return (
    <TeacherApplicationDetailPanel
      application={application}
      auditError={route.auditError}
      auditEvents={route.auditEvents}
      auditState={route.auditState}
      canApprove={route.canApprove}
      canReject={route.canReject}
      decisionDraft={route.decisionDraft}
      decisionError={route.decisionError}
      decisionState={route.decisionState}
      onDecisionDraftChange={route.updateDecisionDraft}
      onRefreshAudit={() => (application ? void route.loadAudit(application.id) : undefined)}
      onSubmitDecision={route.handleDecision}
    />
  );
}
