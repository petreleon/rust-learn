"use client";

import { AlertCircle, Loader2, LogIn, RefreshCw, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { useCallback, useEffect, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { fetchTeachingCourseStudents, type TeacherCourseStudentsResponse } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { StatePanel } from "./StatePanel";
import { StatusLine } from "./StatusLine";
import { StudentProgressView } from "./StudentProgressView";
import { normalizeRouteError } from "./normalizeRouteError";
import { routeNotice } from "./routeNotice";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";

export function TeacherCourseStudentsRoute({ courseId }: { courseId: string }) {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [students, setStudents] = useState<TeacherCourseStudentsResponse | null>(null);

  const loadStudentsRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setStudents(null);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const [nextSession, nextStudents] = await Promise.all([
        fetchCurrentSession({ token }),
        fetchTeachingCourseStudents({ courseId, token }),
      ]);
      setSession(nextSession);
      setStudents(nextStudents);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setStudents(null);
      setError(routeError);
      setLoadState("error");
    }
  }, [courseId]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadStudentsRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadStudentsRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setStudents(null);
    setError(null);
    setLoadState("idle");
  }

  const courseTitle = students?.course.title || "Student progress";
  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { href: `/teach/courses/${courseId}`, label: courseTitle },
        { label: "Students" },
      ]}
      description="Teacher-visible learner roster with honest progress support and reward evidence."
      eyebrow="Teacher students"
      isSignedIn={hasToken || Boolean(session)}
      notice={routeNotice(error)}
      onSignOut={signOut}
      session={session}
      statusItems={
        students ? (
          <>
            <StatusLine icon={<Users size={16} aria-hidden />} label={`${students.total} learner${students.total === 1 ? "" : "s"}`} tone={students.total ? "good" : "neutral"} />
            <StatusLine icon={<Trophy size={16} aria-hidden />} label={students.reward_evidence_supported ? "Reward evidence" : "No reward evidence"} tone={students.reward_evidence_supported ? "good" : "neutral"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      {loadState === "loading" ? (
        <StatePanel
          detail="Loading learner roster, progress support, and reward evidence."
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading student progress"
        />
      ) : null}

      {loadState === "idle" ? (
        <StatePanel
          action={
            <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/students`}>
              <LogIn size={18} aria-hidden />
              Sign in
            </Link>
          }
          detail="Student progress loads from your signed-in teaching session."
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
          <button className={styles.secondaryButton} type="button" onClick={() => void loadStudentsRoute()}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && students ? <StudentProgressView students={students} /> : null}
    </ProductShell>
  );
}
