"use client";

import { useCallback, useEffect, useState } from "react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseStudentsResponse } from "@/lib/teacher/TeacherCourseStudentsResponse";
import { type TeacherRewardCandidate } from "@/lib/teacher/TeacherRewardCandidate";
import { type TeacherRewardCandidateStatusFilter } from "@/lib/teacher/TeacherRewardCandidateStatusFilter";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadCourseRewardCandidates, loadCourseRewardContext } from "../api/courseRewardsApi";
import { canOpenRewardReview } from "../model/canOpenRewardReview";
import { useRewardDecisionSubmit } from "./useRewardDecisionSubmit";
import { normalizeRewardRouteError } from "./normalizeRewardRouteError";

const defaultStatusFilter: TeacherRewardCandidateStatusFilter = "pending_teacher_approval";

export function useTeacherCourseRewardsRoute(courseId: string) {
  const [candidates, setCandidates] = useState<TeacherRewardCandidate[]>([]);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [statusFilter, setStatusFilter] = useState<TeacherRewardCandidateStatusFilter>(defaultStatusFilter);
  const [students, setStudents] = useState<TeacherCourseStudentsResponse | null>(null);

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setCandidates([]);
    setError(null);
    setLoadState(nextLoadState);
    setSession(null);
    setStudents(null);
  }, []);

  const resetForUnauthorized = useCallback(() => {
    clearBrowserSession();
    setHasToken(false);
    clearRoute("error");
  }, [clearRoute]);

  const loadRewardsRoute = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) {
      setHasToken(false);
      clearRoute("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const context = await loadCourseRewardContext({ courseId, token });
      setSession(context.session);
      setStudents(context.students);

      if (!canOpenRewardReview(context.students)) {
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
      const routeError = normalizeRewardRouteError(nextError);
      if (routeError.status === 401) resetForUnauthorized();
      setError(routeError);
      setLoadState("error");
      return;
    }

    try {
      const nextCandidates = await loadCourseRewardCandidates({ courseId, status: statusFilter, token });
      setCandidates(nextCandidates);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRewardRouteError(nextError);
      if (routeError.status === 401) resetForUnauthorized();
      setCandidates([]);
      setError(routeError);
      setLoadState("error");
    }
  }, [clearRoute, courseId, resetForUnauthorized, statusFilter]);

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
    clearBrowserSession();
    setHasToken(false);
    clearRoute("idle");
  }

  return {
    candidates,
    decision,
    error,
    hasToken,
    loadRewardsRoute,
    loadState,
    session,
    setStatusFilter,
    signOut,
    statusFilter,
    students,
  };
}

export type TeacherCourseRewardsRouteController = ReturnType<typeof useTeacherCourseRewardsRoute>;
