"use client";

import { AlertCircle, FileText, Loader2, LogIn, RefreshCw, ShieldCheck, Trophy } from "lucide-react";
import Link from "next/link";
import { ProductShell } from "@/components/product-shell";
import { StatePanel } from "@/features/teacher/shared/route-kit/StatePanel";
import { StatusLine } from "@/features/teacher/shared/route-kit/StatusLine";
import { lifecycleTone } from "@/features/teacher/shared/route-kit/lifecycleTone";
import { routeNotice } from "@/features/teacher/shared/route-kit/routeNotice";
import { statusLabel } from "@/features/teacher/shared/route-kit/statusLabel";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { workspaceSummary } from "../model/workspaceSummary";
import { type TeacherCourseWorkspaceRouteController } from "../route/useTeacherCourseWorkspaceRoute";
import { WorkspaceContent } from "../components/WorkspaceContent";

export function TeacherCourseWorkspaceView({
  courseId,
  route,
}: {
  courseId: string;
  route: TeacherCourseWorkspaceRouteController;
}) {
  const courseTitle = route.workspace?.course.title || "Course workspace";

  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={courseWorkspaceBreadcrumbs(courseTitle)}
      description="Course-scoped teaching workspace for lifecycle, content structure, roster pressure, and reward review readiness."
      eyebrow="Teacher course"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={routeNotice(route.error)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<CourseWorkspaceStatusItems route={route} />}
      title={courseTitle}
    >
      <CourseWorkspaceBody courseId={courseId} route={route} />
    </ProductShell>
  );
}

function CourseWorkspaceBody({
  courseId,
  route,
}: {
  courseId: string;
  route: TeacherCourseWorkspaceRouteController;
}) {
  if (route.loadState === "loading") return <LoadingCourseWorkspace />;
  if (route.loadState === "idle") return <SignedOutCourseWorkspace courseId={courseId} />;
  if (route.loadState === "error" && route.error) return <CourseWorkspaceError route={route} />;
  if (route.loadState === "success" && route.workspace) {
    return <WorkspaceContent courseAction={route.courseAction} workspace={route.workspace} />;
  }
  return null;
}

function CourseWorkspaceStatusItems({ route }: { route: TeacherCourseWorkspaceRouteController }) {
  if (!route.workspace) return null;

  const totals = workspaceSummary(route.workspace);
  const rewardCount = route.workspace.course.reward_queue.pending_teacher_count;

  return (
    <>
      <StatusLine
        icon={<ShieldCheck size={16} aria-hidden />}
        label={statusLabel(route.workspace.course.lifecycle_status)}
        tone={lifecycleTone(route.workspace.course.lifecycle_status)}
      />
      <StatusLine
        icon={<FileText size={16} aria-hidden />}
        label={`${totals.contentCount} content item${totals.contentCount === 1 ? "" : "s"}`}
        tone={totals.contentCount ? "good" : "warn"}
      />
      <StatusLine
        icon={<Trophy size={16} aria-hidden />}
        label={`${rewardCount} reward review${rewardCount === 1 ? "" : "s"}`}
        tone={rewardCount ? "warn" : "neutral"}
      />
    </>
  );
}

function LoadingCourseWorkspace() {
  return (
    <StatePanel
      detail="Loading course lifecycle, teaching permissions, chapters, content, processing signals, roster pressure, and reward queues."
      icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
      title="Loading course workspace"
    />
  );
}

function SignedOutCourseWorkspace({ courseId }: { courseId: string }) {
  return (
    <StatePanel
      action={
        <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}`}>
          <LogIn size={18} aria-hidden />
          Sign in
        </Link>
      }
      detail="Course workspaces load from your signed-in teaching session."
      icon={<LogIn size={22} aria-hidden />}
      title="Sign in required"
    />
  );
}

function CourseWorkspaceError({ route }: { route: TeacherCourseWorkspaceRouteController }) {
  return (
    <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
      <AlertCircle size={18} aria-hidden />
      <span>
        <strong>{route.error?.code}</strong>
        {route.error?.message}
      </span>
      <button className={styles.secondaryButton} type="button" onClick={() => route.loadWorkspace()}>
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function courseWorkspaceBreadcrumbs(courseTitle: string) {
  return [
    { href: "/session", label: "Workspace" },
    { href: "/teach", label: "Teach" },
    { href: "/teach/courses", label: "Courses" },
    { label: courseTitle },
  ];
}
