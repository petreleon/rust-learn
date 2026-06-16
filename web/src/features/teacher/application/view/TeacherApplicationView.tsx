"use client";

import { AlertCircle, FileText, Loader2, LogIn, RefreshCw, ShieldCheck } from "lucide-react";
import Link from "next/link";
import { ProductShell } from "@/components/product-shell";
import styles from "@/app/teach/apply/page.module.css";
import { ApplicationForm } from "../components/ApplicationForm";
import { ApplicationOverview } from "../components/ApplicationOverview";
import { AuditTimeline } from "../components/AuditTimeline";
import { StatePanel } from "../components/StatePanel";
import { StatusLine } from "../components/StatusLine";
import { type TeacherApplicationRouteController } from "../route/useTeacherApplicationRoute";
import { routeNotice } from "./routeNotice";

export function TeacherApplicationView({ route }: { route: TeacherApplicationRouteController }) {
  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { label: "Apply" },
      ]}
      description="Apply for teaching access, track review status, and see reviewer feedback without using the operations console."
      eyebrow="Teacher"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={routeNotice(route.error, route.submitError)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<TeacherApplicationStatusItems route={route} />}
      title="Teacher application"
    >
      <TeacherApplicationBody route={route} />
    </ProductShell>
  );
}

function TeacherApplicationBody({ route }: { route: TeacherApplicationRouteController }) {
  if (route.loadState === "loading") return <LoadingApplication />;
  if (route.loadState === "idle") return <SignedOutApplication />;
  if (route.loadState === "error" && route.error) return <ApplicationRouteError route={route} />;
  if (route.loadState === "success" && route.session) return <LoadedApplication route={route} />;
  return null;
}

function TeacherApplicationStatusItems({ route }: { route: TeacherApplicationRouteController }) {
  return (
    <>
      <StatusLine
        icon={<ShieldCheck size={16} aria-hidden />}
        label={applicationAccessLabel(route)}
        tone={route.canSubmitApplication ? "good" : route.application || route.loadState === "loading" ? "neutral" : "warn"}
      />
      <StatusLine icon={route.statusConfig.icon} label={route.statusConfig.label} tone={route.statusConfig.tone} />
    </>
  );
}

function LoadedApplication({ route }: { route: TeacherApplicationRouteController }) {
  const application = route.application;

  return (
    <>
      <ApplicationOverview
        application={application}
        canSubmitApplication={route.canSubmitApplication}
        onStartNewApplication={() => route.setShowRejectedForm(true)}
        session={route.session!}
        showRejectedForm={route.showRejectedForm}
        statusConfig={route.statusConfig}
        workspaceSummary={route.workspaceSummary}
      />
      {!route.canSubmitApplication && route.formVisible ? <ApplicationPermissionNotice /> : null}
      {route.formVisible && route.canSubmitApplication ? (
        <ApplicationForm
          draft={route.draft}
          onDraftChange={route.setDraft}
          onSubmit={route.handleSubmit}
          selectedCourseLabel={route.selectedCourse?.title || null}
          selectedOrganizationLabel={route.selectedOrganization?.name || null}
          session={route.session!}
          submitState={route.submitState}
          validationErrors={route.validationErrors}
        />
      ) : null}
      {!route.formVisible && application?.status === "needs_changes" ? <ChangesRequestedNotice /> : null}
      {application ? (
        <AuditTimeline auditEvents={route.snapshot.audit_events} decisionReason={application.decision_reason} />
      ) : null}
    </>
  );
}

function LoadingApplication() {
  return (
    <StatePanel
      detail="Resolving your current session, teacher application state, and review history."
      icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
      title="Loading application"
    />
  );
}

function SignedOutApplication() {
  return (
    <StatePanel
      action={
        <Link className={styles.primaryLink} href="/login?redirect=/teach/apply">
          <LogIn size={18} aria-hidden />
          Sign in
        </Link>
      }
      detail="Teacher applications use your current RustLearn account and saved session."
      icon={<LogIn size={22} aria-hidden />}
      title="Sign in required"
    />
  );
}

function ApplicationRouteError({ route }: { route: TeacherApplicationRouteController }) {
  return (
    <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
      <AlertCircle size={18} aria-hidden />
      <span>
        <strong>{route.error?.code}</strong>
        {route.error?.message}
      </span>
      <button className={styles.secondaryButton} type="button" onClick={() => route.loadApplication()}>
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function ApplicationPermissionNotice() {
  return (
    <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <AlertCircle size={20} aria-hidden />
        <h2>Application permission needed</h2>
      </div>
      <p className={styles.muted}>
        Your current session can view this page, but it does not include teacher application submission access.
      </p>
    </section>
  );
}

function ChangesRequestedNotice() {
  return (
    <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <FileText size={20} aria-hidden />
        <h2>Changes requested</h2>
      </div>
      <p className={styles.muted}>
        Reviewer feedback is visible below. Editing and resubmission are blocked until the application update contract exists.
      </p>
    </section>
  );
}

function applicationAccessLabel(route: TeacherApplicationRouteController) {
  if (route.canSubmitApplication) return "Application access";
  if (route.application) return "Application visible";
  if (route.loadState === "loading") return "Resolving access";
  return "Access needed";
}
