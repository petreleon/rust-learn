"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace } from "@/lib/admin/buildPlatformAdminWorkspace";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminDashboardSession } from "../api/dashboardApi";
import {
  canApproveRewardAmount,
  canExportPlatformData,
  canManageRewardFraud,
  canViewPlatformSummary,
  canViewRewardAudit,
  emptyPlatformAdminWorkspace,
} from "../model/dashboardWorkspace";
import { normalizeAdminDashboardRouteError } from "./normalizeAdminDashboardRouteError";
import { useAdminDashboardCsvDownload } from "./useAdminDashboardCsvDownload";
import { useAdminDashboardSections } from "./useAdminDashboardSections";

function isExpiredSession(error: RouteError) {
  return error.status === 401 || error.status === 404;
}

export function useAdminDashboardRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const workspace = useMemo(
    () => (session ? buildPlatformAdminWorkspace(session) : emptyPlatformAdminWorkspace),
    [session],
  );
  const allowed = session ? hasPlatformAdminAccess(session) : false;
  const canViewSummary = canViewPlatformSummary(workspace);
  const canViewRewardOperations = canViewRewardAudit(workspace);
  const canApproveRewards = canApproveRewardAmount(workspace);
  const canExportData = canExportPlatformData(workspace);
  const canManageFraud = canManageRewardFraud(workspace);
  const csv = useAdminDashboardCsvDownload();
  const sections = useAdminDashboardSections({
    allowed,
    canViewRewardOperations,
    canViewSummary,
    session,
  });
  const { loadDashboard } = sections;

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
      setSession(await loadAdminDashboardSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeAdminDashboardRouteError(
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
    if (session && allowed) {
      const timeout = window.setTimeout(() => loadDashboard(), 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [allowed, loadDashboard, session]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    allowed,
    canApproveRewards,
    canExportData,
    canManageFraud,
    canViewRewardOperations,
    canViewSummary,
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
    workspace,
    ...csv,
    ...sections,
  };
}

export type AdminDashboardRouteController = ReturnType<typeof useAdminDashboardRoute>;
