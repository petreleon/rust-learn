"use client";

import { useCallback, useEffect, useState } from "react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadTeacherCourseWorkspace } from "../api/courseWorkspaceApi";
import { hasProcessingContent } from "../model/workspaceSummary";
import { normalizeCourseWorkspaceRouteError } from "./normalizeCourseWorkspaceRouteError";

export function useTeacherCourseWorkspaceRoute(courseId: string) {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [workspace, setWorkspace] = useState<TeacherCourseWorkspaceResponse | null>(null);

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setError(null);
    setLoadState(nextLoadState);
    setSession(null);
    setWorkspace(null);
  }, []);

  const loadWorkspace = useCallback(async () => {
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
      const next = await loadTeacherCourseWorkspace({ courseId, token });
      setSession(next.session);
      setWorkspace(next.workspace);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeCourseWorkspaceRouteError(nextError);
      if (routeError.status === 401) {
        clearBrowserSession();
        setHasToken(false);
      }
      setSession(null);
      setWorkspace(null);
      setError(routeError);
      setLoadState("error");
    }
  }, [clearRoute, courseId]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadWorkspace(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadWorkspace]);

  useEffect(() => {
    if (loadState !== "success" || !workspace || !hasProcessingContent(workspace)) return;
    const interval = window.setInterval(() => void loadWorkspace(), 30000);
    return () => window.clearInterval(interval);
  }, [loadState, workspace, loadWorkspace]);

  const signOut = useCallback(() => {
    clearBrowserSession();
    setHasToken(false);
    clearRoute("idle");
  }, [clearRoute]);

  return {
    error,
    hasToken,
    loadState,
    loadWorkspace,
    session,
    signOut,
    workspace,
  };
}

export type TeacherCourseWorkspaceRouteController = ReturnType<typeof useTeacherCourseWorkspaceRoute>;
