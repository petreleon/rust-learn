"use client";

import { type FormEvent, useCallback, useEffect, useState } from "react";
import { clearStoredSessionToken } from "@/lib/session/clearStoredSessionToken";
import { readStoredSessionToken } from "@/lib/session/readStoredSessionToken";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseEnrollmentWorkspaceResponse } from "@/lib/teacher/TeacherCourseEnrollmentWorkspaceResponse";
import { type TeacherCourseJoinRequestItem } from "@/lib/teacher/TeacherCourseJoinRequestItem";
import { type TeacherCourseRosterLearner } from "@/lib/teacher/TeacherCourseRosterLearner";
import { type ActionState } from "@/components/teacher-routes/ActionState";
import { type DecisionDraft } from "@/components/teacher-routes/DecisionDraft";
import { type EnrollmentStatusFilter } from "@/components/teacher-routes/EnrollmentStatusFilter";
import { type LoadState } from "@/components/teacher-routes/LoadState";
import { type RouteError } from "@/components/teacher-routes/RouteError";
import { defaultDecisionDraft } from "@/components/teacher-routes/defaultDecisionDraft";
import { normalizeRouteError } from "@/components/teacher-routes/normalizeRouteError";
import {
  decideEnrollmentRequest,
  loadTeacherCourseEnrollments,
  removeEnrollmentLearner,
} from "../api/enrollmentApi";

export function useTeacherCourseEnrollmentsRoute(courseId: string) {
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [confirmRemovalUserId, setConfirmRemovalUserId] = useState<number | null>(null);
  const [decisionDrafts, setDecisionDrafts] = useState<Record<number, DecisionDraft>>({});
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [statusFilter, setStatusFilter] = useState<EnrollmentStatusFilter>("open");
  const [workspace, setWorkspace] = useState<TeacherCourseEnrollmentWorkspaceResponse | null>(null);

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setSession(null);
    setWorkspace(null);
    setError(null);
    setLoadState(nextLoadState);
  }, []);

  const loadEnrollmentRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      clearRoute("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const next = await loadTeacherCourseEnrollments({ courseId, status: statusFilter, token });
      setSession(next.session);
      setWorkspace(next.workspace);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setWorkspace(null);
      setError(routeError);
      setLoadState("error");
    }
  }, [clearRoute, courseId, statusFilter]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadEnrollmentRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadEnrollmentRoute]);

  useEffect(() => {
    function handleVisible() {
      if (document.visibilityState === "visible") {
        void loadEnrollmentRoute();
      }
    }

    document.addEventListener("visibilitychange", handleVisible);
    return () => document.removeEventListener("visibilitychange", handleVisible);
  }, [loadEnrollmentRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    clearRoute("idle");
  }

  function updateDecisionDraft(requestId: number, draft: DecisionDraft) {
    setDecisionDrafts((current) => ({ ...current, [requestId]: draft }));
  }

  async function submitDecision(request: TeacherCourseJoinRequestItem, event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const draft = decisionDrafts[request.id] || defaultDecisionDraft;
    if (!token) {
      setActionMessage("Sign in again before deciding enrollment requests.");
      return;
    }

    setActionState("saving");
    setActionMessage(null);

    try {
      await decideEnrollmentRequest({
        courseId,
        payload: { decision_reason: draft.reason.trim() || null, status: draft.status },
        requestId: request.id,
        token,
      });
      setDecisionDrafts((current) => {
        const next = { ...current };
        delete next[request.id];
        return next;
      });
      setActionMessage("Enrollment request updated.");
      await loadEnrollmentRoute();
    } catch (nextError) {
      setActionMessage(normalizeRouteError(nextError).message);
    } finally {
      setActionState("idle");
    }
  }

  async function removeLearner(learner: TeacherCourseRosterLearner) {
    const token = readStoredSessionToken();
    if (!token) {
      setActionMessage("Sign in again before changing roster access.");
      return;
    }

    if (confirmRemovalUserId !== learner.user.id) {
      setConfirmRemovalUserId(learner.user.id);
      setActionMessage(`Confirm removal for ${learner.user.name}.`);
      return;
    }

    setActionState("saving");
    setActionMessage(null);

    try {
      await removeEnrollmentLearner({ courseId, token, userId: learner.user.id });
      setConfirmRemovalUserId(null);
      setActionMessage("Learner removed from the course roster.");
      await loadEnrollmentRoute();
    } catch (nextError) {
      setActionMessage(normalizeRouteError(nextError).message);
    } finally {
      setActionState("idle");
    }
  }

  return {
    actionMessage,
    actionState,
    confirmRemovalUserId,
    decisionDrafts,
    error,
    hasToken,
    loadEnrollmentRoute,
    loadState,
    removeLearner,
    session,
    setStatusFilter,
    signOut,
    statusFilter,
    submitDecision,
    updateDecisionDraft,
    workspace,
  };
}

export type TeacherCourseEnrollmentsRouteController = ReturnType<typeof useTeacherCourseEnrollmentsRoute>;
