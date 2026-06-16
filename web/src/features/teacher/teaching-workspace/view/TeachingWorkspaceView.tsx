"use client";

import { AlertCircle, BookOpen, Loader2, LogIn, RefreshCw, ShieldCheck, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { type FormEvent } from "react";
import { ProductShell } from "@/components/product-shell";
import { StatePanel } from "@/components/teacher-routes/StatePanel";
import { StatusLine } from "@/components/teacher-routes/StatusLine";
import styles from "@/components/teacher-routes.module.css";
import { CoursesView } from "../components/CoursesView";
import { DashboardView } from "../components/DashboardView";
import { type TeachingWorkspaceRouteController } from "../route/useTeachingWorkspaceRoute";
import { teacherBreadcrumbs, teacherDescription, teachingRouteNotice } from "./teachingWorkspaceShell";

export function TeachingWorkspaceView({ route }: { route: TeachingWorkspaceRouteController }) {
  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={teacherBreadcrumbs(route.view)}
      description={teacherDescription(route.view)}
      eyebrow="Teacher"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={teachingRouteNotice(route.error)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<TeacherStatusItems route={route} />}
      title={route.view === "courses" ? "Teaching courses" : "Teaching workspace"}
    >
      <TeachingWorkspaceBody route={route} />
    </ProductShell>
  );
}

function TeachingWorkspaceBody({ route }: { route: TeachingWorkspaceRouteController }) {
  if (route.loadState === "loading") return <LoadingTeachingWorkspace />;
  if (route.loadState === "idle") return <SignedOutTeachingWorkspace view={route.view} />;
  if (route.loadState === "error" && route.error) return <TeacherRouteError route={route} />;
  if (route.loadState === "success" && route.session) return <LoadedTeachingWorkspace route={route} />;
  return null;
}

function LoadedTeachingWorkspace({ route }: { route: TeachingWorkspaceRouteController }) {
  return (
    <>
      {!route.canUseTeacherSurface ? <NoTeachingAccessPanel /> : null}
      {route.view === "dashboard" ? (
        <DashboardView
          application={route.application}
          canSubmitApplication={route.canSubmitApplication}
          courses={route.previewCourses}
          hasMoreCourses={route.courses.total > route.previewCourses.length}
          totals={route.totals}
        />
      ) : (
        <CoursesView
          courses={route.previewCourses}
          onApplyFilters={(event: FormEvent<HTMLFormElement>) => {
            event.preventDefault();
            route.applyFilters();
          }}
          onQueryChange={route.setQuery}
          query={route.query}
          total={route.courses.total}
        />
      )}
    </>
  );
}

function TeacherStatusItems({ route }: { route: TeachingWorkspaceRouteController }) {
  return (
    <>
      <StatusLine
        icon={<BookOpen size={16} aria-hidden />}
        label={`${route.courses.total} teaching course${route.courses.total === 1 ? "" : "s"}`}
        tone={route.courses.total ? "good" : "neutral"}
      />
      <StatusLine
        icon={<Users size={16} aria-hidden />}
        label={`${route.totals.pendingEnrollmentCount} enrollment request${route.totals.pendingEnrollmentCount === 1 ? "" : "s"}`}
        tone={route.totals.pendingEnrollmentCount ? "warn" : "neutral"}
      />
      <StatusLine
        icon={<Trophy size={16} aria-hidden />}
        label={`${route.totals.pendingRewardCount} reward review${route.totals.pendingRewardCount === 1 ? "" : "s"}`}
        tone={route.totals.pendingRewardCount ? "warn" : "neutral"}
      />
    </>
  );
}

function LoadingTeachingWorkspace() {
  return (
    <StatePanel
      detail="Resolving teaching courses, enrollment queues, reward candidates, and application status."
      icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
      title="Loading teaching workspace"
    />
  );
}

function SignedOutTeachingWorkspace({ view }: { view: "dashboard" | "courses" }) {
  return (
    <StatePanel
      action={
        <Link className={styles.primaryLink} href={`/login?redirect=${view === "courses" ? "/teach/courses" : "/teach"}`}>
          <LogIn size={18} aria-hidden />
          Sign in
        </Link>
      }
      detail="Teaching work loads from your signed-in RustLearn session."
      icon={<LogIn size={22} aria-hidden />}
      title="Sign in required"
    />
  );
}

function TeacherRouteError({ route }: { route: TeachingWorkspaceRouteController }) {
  return (
    <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
      <AlertCircle size={18} aria-hidden />
      <span>
        <strong>{route.error?.code}</strong>
        {route.error?.message}
      </span>
      <button className={styles.secondaryButton} type="button" onClick={() => route.applyFilters()}>
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function NoTeachingAccessPanel() {
  return (
    <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <h2>No teaching access yet</h2>
      </div>
      <p className={styles.muted}>
        This session has no teaching courses, no visible teacher application, and no application submission permission.
      </p>
    </section>
  );
}
