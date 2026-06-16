"use client";

import { useCallback, useEffect, useState } from "react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { type ActionState } from "@/shared/route-state/ActionState";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadTeacherCourseWorkspace,
  saveTeacherCourseLifecycle,
  saveTeacherCourseSettings,
} from "../api/courseWorkspaceApi";
import { courseSettingsValidation, type CourseSettingsDraft } from "../model/courseSettingsModel";
import { hasProcessingContent } from "../model/workspaceSummary";
import { normalizeCourseWorkspaceRouteError } from "./normalizeCourseWorkspaceRouteError";

export function useTeacherCourseWorkspaceRoute(courseId: string) {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [workspace, setWorkspace] = useState<TeacherCourseWorkspaceResponse | null>(null);
  const [courseActionMessage, setCourseActionMessage] = useState<string | null>(null);
  const [courseActionState, setCourseActionState] = useState<ActionState>("idle");

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

  const runCourseAction = useCallback(
    async (action: (token: string) => Promise<unknown>, successMessage: string) => {
      const token = readBrowserSessionToken();
      if (!token) {
        setHasToken(false);
        clearRoute("idle");
        return;
      }

      setCourseActionState("saving");
      setCourseActionMessage(null);
      try {
        await action(token);
        setCourseActionMessage(successMessage);
        await loadWorkspace();
      } catch (nextError) {
        const routeError = normalizeCourseWorkspaceRouteError(nextError);
        if (routeError.status === 401) {
          clearBrowserSession();
          setHasToken(false);
          clearRoute("idle");
        }
        setCourseActionMessage(routeError.message);
      } finally {
        setCourseActionState("idle");
      }
    },
    [clearRoute, loadWorkspace],
  );

  const submitCourseSettings = useCallback(
    async (draft: CourseSettingsDraft) => {
      const validation = courseSettingsValidation(draft);
      if (validation) {
        setCourseActionMessage(validation);
        return;
      }
      await runCourseAction(
        (token) => saveTeacherCourseSettings({ courseId, draft, token }),
        "Course settings updated.",
      );
    },
    [courseId, runCourseAction],
  );

  const submitCourseLifecycle = useCallback(
    async (status: string) => {
      await runCourseAction(
        (token) => saveTeacherCourseLifecycle({ courseId, status, token }),
        "Course lifecycle updated.",
      );
    },
    [courseId, runCourseAction],
  );

  return {
    courseAction: {
      actionMessage: courseActionMessage,
      actionState: courseActionState,
      submitCourseLifecycle,
      submitCourseSettings,
    },
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
