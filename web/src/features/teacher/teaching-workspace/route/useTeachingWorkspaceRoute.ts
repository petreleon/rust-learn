"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { hasTeacherApplicationAccess } from "@/lib/access";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherApplicationSnapshot } from "@/lib/teacher/TeacherApplicationSnapshot";
import { type TeacherCoursesResponse } from "@/lib/teacher/TeacherCoursesResponse";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadTeachingWorkspaceData } from "../api/teachingWorkspaceApi";
import { defaultCourseQuery, type CourseQuery } from "../model/CourseQuery";
import { dashboardTotals } from "../model/dashboardTotals";
import { emptyApplicationSnapshot, emptyCourses } from "../model/emptyTeacherWorkspace";
import { type TeacherRouteView } from "../model/TeacherRouteView";
import { normalizeTeachingWorkspaceRouteError } from "./normalizeTeachingWorkspaceRouteError";

export function useTeachingWorkspaceRoute({ view }: { view: TeacherRouteView }) {
  const [applicationSnapshot, setApplicationSnapshot] =
    useState<TeacherApplicationSnapshot>(emptyApplicationSnapshot);
  const [courses, setCourses] = useState<TeacherCoursesResponse>(emptyCourses);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [query, setQuery] = useState<CourseQuery>(defaultCourseQuery);
  const [session, setSession] = useState<CurrentSession | null>(null);

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setApplicationSnapshot(emptyApplicationSnapshot);
    setCourses(emptyCourses);
    setError(null);
    setHasToken(false);
    setLoadState(nextLoadState);
    setSession(null);
  }, []);

  const loadTeacherRoute = useCallback(
    async (nextQuery: CourseQuery) => {
      const token = readBrowserSessionToken();
      if (!token) {
        clearRoute("idle");
        return;
      }

      setHasToken(true);
      setLoadState("loading");
      setError(null);

      try {
        const data = await loadTeachingWorkspaceData({ query: nextQuery, token });
        setSession(data.session);
        setCourses(data.courses);
        setApplicationSnapshot(data.applicationSnapshot);
        setLoadState("success");
      } catch (nextError) {
        const routeError = normalizeTeachingWorkspaceRouteError(nextError);
        if (routeError.status === 401 || routeError.status === 404) {
          clearBrowserSession();
          setHasToken(false);
        }
        setSession(null);
        setApplicationSnapshot(emptyApplicationSnapshot);
        setCourses(emptyCourses);
        setError(routeError);
        setLoadState("error");
      }
    },
    [clearRoute],
  );

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadTeacherRoute(defaultCourseQuery), 0);
    return () => window.clearTimeout(timeout);
  }, [loadTeacherRoute]);

  const totals = useMemo(() => dashboardTotals(courses.courses), [courses.courses]);
  const canSubmitApplication = session ? hasTeacherApplicationAccess(session) : false;
  const application = applicationSnapshot.application;
  const canUseTeacherSurface = courses.total > 0 || canSubmitApplication || Boolean(application);
  const previewCourses = view === "dashboard" ? courses.courses.slice(0, 3) : courses.courses;

  const applyFilters = useCallback(() => {
    void loadTeacherRoute(query);
  }, [loadTeacherRoute, query]);

  const signOut = useCallback(() => {
    clearBrowserSession();
    clearRoute("idle");
  }, [clearRoute]);

  return {
    application,
    applicationSnapshot,
    applyFilters,
    canSubmitApplication,
    canUseTeacherSurface,
    courses,
    error,
    hasToken,
    loadState,
    loadTeacherRoute,
    previewCourses,
    query,
    session,
    setQuery,
    signOut,
    totals,
    view,
  };
}

export type TeachingWorkspaceRouteController = ReturnType<typeof useTeachingWorkspaceRoute>;
