"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace } from "@/lib/admin/buildPlatformAdminWorkspace";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminTeacherApplicationSession } from "../api/teacherApplicationsApi";
import {
  canApproveTeacherApplication,
  canRejectTeacherApplication,
  canReviewTeacherApplications,
  emptyTeacherApplicationWorkspace,
  findTeacherApplicationCapability,
} from "../model/teacherApplicationAccessModel";
import { normalizeAdminTeacherApplicationRouteError } from "./normalizeAdminTeacherApplicationRouteError";
import { useTeacherApplicationAudit } from "./useTeacherApplicationAudit";
import { useTeacherApplicationDecision } from "./useTeacherApplicationDecision";
import { useTeacherApplicationList } from "./useTeacherApplicationList";

function isExpiredSession(error: RouteError) {
  return error.status === 401 || error.status === 404;
}

export function useAdminTeacherApplicationsRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const workspace = useMemo(
    () => (session ? buildPlatformAdminWorkspace(session) : emptyTeacherApplicationWorkspace),
    [session],
  );
  const allowed = session ? hasPlatformAdminAccess(session) : false;
  const canApprove = canApproveTeacherApplication(workspace);
  const canReject = canRejectTeacherApplication(workspace);
  const canReview = canReviewTeacherApplications(workspace);
  const teacherApplicationCapability = findTeacherApplicationCapability(workspace);
  const list = useTeacherApplicationList({ allowed, canReview, session });
  const audit = useTeacherApplicationAudit({ canReview });
  const { loadAudit, resetAudit } = audit;
  const { loadApplications, selectedApplication } = list;
  const decision = useTeacherApplicationDecision({
    onApplicationChanged: loadApplications,
    onConflict: loadAudit,
    onDecisionSaved: loadAudit,
    selectedApplication,
  });

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setError(null);
    setHasToken(false);
    setLoadState(nextLoadState);
    setSession(null);
  }, []);

  const loadSession = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) {
      clearRoute("idle");
      return;
    }

    setError(null);
    setHasToken(true);
    setLoadState("loading");

    try {
      setSession(await loadAdminTeacherApplicationSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeAdminTeacherApplicationRouteError(
        nextError,
        "Platform admin session could not be loaded.",
      );
      if (isExpiredSession(routeError)) {
        clearBrowserSession();
        setHasToken(false);
      }
      setError(routeError);
      setLoadState("error");
      setSession(null);
    }
  }, [clearRoute]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  useEffect(() => {
    if (session && allowed && canReview) {
      const timeout = window.setTimeout(() => void loadApplications(), 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [allowed, canReview, loadApplications, session]);

  useEffect(() => {
    if (!selectedApplication?.id) {
      const timeout = window.setTimeout(() => resetAudit(), 0);
      return () => window.clearTimeout(timeout);
    }
    const timeout = window.setTimeout(() => void loadAudit(selectedApplication.id), 0);
    return () => window.clearTimeout(timeout);
  }, [selectedApplication?.id, loadAudit, resetAudit]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    allowed,
    canApprove,
    canReject,
    canReview,
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
    teacherApplicationCapability,
    workspace,
    ...list,
    ...audit,
    ...decision,
  };
}

export type AdminTeacherApplicationsRouteController = ReturnType<
  typeof useAdminTeacherApplicationsRoute
>;
