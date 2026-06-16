"use client";

import { AlertCircle, Clock3, Loader2, LogIn, RefreshCw, Users } from "lucide-react";
import Link from "next/link";
import { ProductShell } from "@/components/product-shell";
import { EnrollmentWorkspaceView } from "@/components/teacher-routes/EnrollmentWorkspaceView";
import { StatePanel } from "@/components/teacher-routes/StatePanel";
import { StatusLine } from "@/components/teacher-routes/StatusLine";
import { routeNotice } from "@/components/teacher-routes/routeNotice";
import styles from "@/components/teacher-routes.module.css";
import {
  courseEnrollmentTitle,
  joinRequestCountLabel,
  rosterCountLabel,
} from "../model/courseEnrollmentLabels";
import { type TeacherCourseEnrollmentsRouteController } from "../route/useTeacherCourseEnrollmentsRoute";

export function TeacherCourseEnrollmentsView({
  courseId,
  route,
}: {
  courseId: string;
  route: TeacherCourseEnrollmentsRouteController;
}) {
  const courseTitle = courseEnrollmentTitle(route.workspace);

  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { href: `/teach/courses/${courseId}`, label: courseTitle },
        { label: "Enrollments" },
      ]}
      description="Review course join requests and roster access without raw learner ids."
      eyebrow="Teacher enrollments"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={routeNotice(route.error)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={route.workspace ? <EnrollmentStatusItems workspace={route.workspace} /> : null}
      title={courseTitle}
    >
      {route.loadState === "loading" ? <EnrollmentLoadingState /> : null}
      {route.loadState === "idle" ? <EnrollmentSignedOutState courseId={courseId} /> : null}
      {route.loadState === "error" && route.error ? (
        <EnrollmentErrorState errorCode={route.error.code} message={route.error.message} onRetry={route.loadEnrollmentRoute} />
      ) : null}
      {route.loadState === "success" && route.workspace ? (
        <EnrollmentWorkspaceView
          actionMessage={route.actionMessage}
          actionState={route.actionState}
          confirmRemovalUserId={route.confirmRemovalUserId}
          decisionDrafts={route.decisionDrafts}
          onDecisionDraftChange={route.updateDecisionDraft}
          onRefresh={route.loadEnrollmentRoute}
          onRemoveLearner={route.removeLearner}
          onStatusFilterChange={route.setStatusFilter}
          onSubmitDecision={route.submitDecision}
          statusFilter={route.statusFilter}
          workspace={route.workspace}
        />
      ) : null}
    </ProductShell>
  );
}

function EnrollmentStatusItems({
  workspace,
}: {
  workspace: NonNullable<TeacherCourseEnrollmentsRouteController["workspace"]>;
}) {
  return (
    <>
      <StatusLine
        icon={<Clock3 size={16} aria-hidden />}
        label={joinRequestCountLabel(workspace)}
        tone={workspace.join_requests.total ? "warn" : "neutral"}
      />
      <StatusLine
        icon={<Users size={16} aria-hidden />}
        label={rosterCountLabel(workspace)}
        tone={workspace.roster.total ? "good" : "neutral"}
      />
    </>
  );
}

function EnrollmentLoadingState() {
  return (
    <StatePanel
      detail="Loading enrollment requests, roster access, and course-scoped permissions."
      icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
      title="Loading enrollments"
    />
  );
}

function EnrollmentSignedOutState({ courseId }: { courseId: string }) {
  return (
    <StatePanel
      action={
        <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/enrollments`}>
          <LogIn size={18} aria-hidden />
          Sign in
        </Link>
      }
      detail="Enrollment management loads from your signed-in teaching session."
      icon={<LogIn size={22} aria-hidden />}
      title="Sign in required"
    />
  );
}

function EnrollmentErrorState({
  errorCode,
  message,
  onRetry,
}: {
  errorCode: string;
  message: string;
  onRetry: () => void;
}) {
  return (
    <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
      <AlertCircle size={18} aria-hidden />
      <span>
        <strong>{errorCode}</strong> {message}
      </span>
      <button className={styles.secondaryButton} type="button" onClick={onRetry}>
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}
