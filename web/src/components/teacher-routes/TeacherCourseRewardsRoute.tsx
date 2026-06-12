"use client";

import { AlertCircle, Clock3, Loader2, LogIn, RefreshCw, Trophy } from "lucide-react";
import Link from "next/link";
import { useCallback, useEffect, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { fetchTeacherRewardCandidates, fetchTeachingCourseStudents, type TeacherCourseStudentsResponse, type TeacherRewardCandidate, type TeacherRewardCandidateStatusFilter } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { RewardReviewView } from "./RewardReviewView";
import { StatePanel } from "./StatePanel";
import { StatusLine } from "./StatusLine";
import { normalizeRouteError } from "./normalizeRouteError";
import { routeNotice } from "./routeNotice";
import { useRewardDecisionSubmit } from "./useRewardDecisionSubmit";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";

function canOpenRewardReview(students: TeacherCourseStudentsResponse) {
  return students.course.permissions.can_view_reward_candidates || students.course.permissions.can_approve_reward_candidates;
}

export function TeacherCourseRewardsRoute({ courseId }: { courseId: string }) {
  const [candidates, setCandidates] = useState<TeacherRewardCandidate[]>([]);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [statusFilter, setStatusFilter] = useState<TeacherRewardCandidateStatusFilter>("pending_teacher_approval");
  const [students, setStudents] = useState<TeacherCourseStudentsResponse | null>(null);

  const resetForUnauthorized = useCallback(() => {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setStudents(null);
    setCandidates([]);
  }, []);

  const loadRewardsRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setStudents(null);
      setCandidates([]);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);
    try {
      setSession(await fetchCurrentSession({ token }));
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) resetForUnauthorized();
      setError(routeError);
      setLoadState("error");
      return;
    }

    try {
      const nextStudents = await fetchTeachingCourseStudents({ courseId, token });
      setStudents(nextStudents);
      if (!canOpenRewardReview(nextStudents)) {
        setCandidates([]);
        setError({
          code: "permission_denied",
          message: "Reward review requires course reward candidate permission for this course.",
          status: 403,
        });
        setLoadState("error");
        return;
      }
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) resetForUnauthorized();
      setError(routeError);
      setLoadState("error");
      return;
    }

    try {
      setCandidates(await fetchTeacherRewardCandidates({ courseId, status: statusFilter, token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) resetForUnauthorized();
      setCandidates([]);
      setError(routeError);
      setLoadState("error");
    }
  }, [courseId, resetForUnauthorized, statusFilter]);

  const decision = useRewardDecisionSubmit({ courseId, onReload: loadRewardsRoute });

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRewardsRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRewardsRoute]);

  useEffect(() => {
    function handleVisible() {
      if (document.visibilityState === "visible") void loadRewardsRoute();
    }
    document.addEventListener("visibilitychange", handleVisible);
    return () => document.removeEventListener("visibilitychange", handleVisible);
  }, [loadRewardsRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setStudents(null);
    setCandidates([]);
    setError(null);
    setLoadState("idle");
  }

  const courseTitle = students?.course.title || "Reward review";
  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { href: `/teach/courses/${courseId}`, label: courseTitle },
        { label: "Rewards" },
      ]}
      description="Review course-scoped reward evidence and apply teacher decisions without platform payout controls."
      eyebrow="Teacher rewards"
      isSignedIn={hasToken || Boolean(session)}
      notice={routeNotice(error)}
      onSignOut={signOut}
      session={session}
      statusItems={
        students ? (
          <>
            <StatusLine icon={<Clock3 size={16} aria-hidden />} label={`${students.course.reward_queue.pending_teacher_count} pending`} tone={students.course.reward_queue.pending_teacher_count ? "warn" : "neutral"} />
            <StatusLine icon={<Trophy size={16} aria-hidden />} label={`${candidates.length} shown`} tone={candidates.length ? "good" : "neutral"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      {loadState === "loading" ? <StatePanel detail="Loading reward candidates, learner context, and course-scoped permissions." icon={<Loader2 className={styles.spin} size={22} aria-hidden />} title="Loading reward review" /> : null}
      {loadState === "idle" ? <StatePanel action={<Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/rewards`}><LogIn size={18} aria-hidden />Sign in</Link>} detail="Reward review loads from your signed-in teaching session." icon={<LogIn size={22} aria-hidden />} title="Sign in required" /> : null}
      {loadState === "error" && error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span><strong>{error.code}</strong> {error.message}</span>
          {students ? <Link className={styles.secondaryLink} href={`/teach/courses/${courseId}`}>Course workspace</Link> : null}
          <button className={styles.secondaryButton} type="button" onClick={() => void loadRewardsRoute()}><RefreshCw size={17} aria-hidden />Retry</button>
        </section>
      ) : null}
      {loadState === "success" && students ? (
        <RewardReviewView actionMessage={decision.actionMessage} actionState={decision.actionState} candidates={candidates} decisionDrafts={decision.decisionDrafts} onDecisionDraftChange={decision.updateDecisionDraft} onRefresh={() => void loadRewardsRoute()} onStatusFilterChange={setStatusFilter} onSubmitDecision={decision.submitRewardDecision} statusFilter={statusFilter} students={students} />
      ) : null}
    </ProductShell>
  );
}
