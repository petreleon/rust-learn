"use client";

import { AlertCircle, FileText, Loader2, LogIn, RefreshCw, ShieldCheck, Trophy } from "lucide-react";
import Link from "next/link";
import { useCallback, useEffect, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { fetchTeachingCourseWorkspace, type TeacherCourseWorkspaceResponse } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { StatePanel } from "./StatePanel";
import { StatusLine } from "./StatusLine";
import { WorkspaceView } from "./WorkspaceView";
import { lifecycleTone } from "./lifecycleTone";
import { normalizeRouteError } from "./normalizeRouteError";
import { routeNotice } from "./routeNotice";
import { statusLabel } from "./statusLabel";
import { workspaceSummary } from "./workspaceSummary";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";

export function TeacherCourseWorkspaceRoute({ courseId }: { courseId: string }) {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [workspace, setWorkspace] = useState<TeacherCourseWorkspaceResponse | null>(null);

  const loadWorkspace = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setWorkspace(null);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const [nextSession, nextWorkspace] = await Promise.all([
        fetchCurrentSession({ token }),
        fetchTeachingCourseWorkspace({ courseId, token }),
      ]);
      setSession(nextSession);
      setWorkspace(nextWorkspace);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        if (routeError.status === 401) {
          clearStoredSessionToken();
          setHasToken(false);
        }
      }
      setSession(null);
      setWorkspace(null);
      setError(routeError);
      setLoadState("error");
    }
  }, [courseId]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadWorkspace(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadWorkspace]);

  useEffect(() => {
    if (loadState !== "success" || !workspace) return;
    const hasProcessing = workspace.chapters.some((chapter) =>
      chapter.contents.some((content) => content.display_state === "processing"),
    );
    if (!hasProcessing) return;
    const interval = window.setInterval(() => void loadWorkspace(), 30000);
    return () => window.clearInterval(interval);
  }, [loadState, workspace, loadWorkspace]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setWorkspace(null);
    setError(null);
    setLoadState("idle");
  }

  const workspaceTotals = workspace ? workspaceSummary(workspace) : null;
  const courseTitle = workspace?.course.title || "Course workspace";

  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { label: courseTitle },
      ]}
      description="Course-scoped teaching workspace for lifecycle, content structure, roster pressure, and reward review readiness."
      eyebrow="Teacher course"
      isSignedIn={hasToken || Boolean(session)}
      notice={routeNotice(error)}
      onSignOut={signOut}
      session={session}
      statusItems={
        workspace && workspaceTotals ? (
          <>
            <StatusLine icon={<ShieldCheck size={16} aria-hidden />} label={statusLabel(workspace.course.lifecycle_status)} tone={lifecycleTone(workspace.course.lifecycle_status)} />
            <StatusLine icon={<FileText size={16} aria-hidden />} label={`${workspaceTotals.contentCount} content item${workspaceTotals.contentCount === 1 ? "" : "s"}`} tone={workspaceTotals.contentCount ? "good" : "warn"} />
            <StatusLine icon={<Trophy size={16} aria-hidden />} label={`${workspace.course.reward_queue.pending_teacher_count} reward review${workspace.course.reward_queue.pending_teacher_count === 1 ? "" : "s"}`} tone={workspace.course.reward_queue.pending_teacher_count ? "warn" : "neutral"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      {loadState === "loading" ? (
        <StatePanel
          detail="Loading course lifecycle, teaching permissions, chapters, content, processing signals, roster pressure, and reward queues."
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading course workspace"
        />
      ) : null}

      {loadState === "idle" ? (
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
      ) : null}

      {loadState === "error" && error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <button className={styles.secondaryButton} type="button" onClick={() => void loadWorkspace()}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && workspace ? <WorkspaceView workspace={workspace} /> : null}
    </ProductShell>
  );
}
