"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace } from "@/lib/admin/buildPlatformAdminWorkspace";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminWalletSession } from "../api/walletsApi";
import {
  canExportWalletCredits,
  canViewWalletAudit,
  emptyPlatformWalletWorkspace,
  findPlatformCapability,
} from "../model/walletAccessModel";
import { normalizeAdminWalletRouteError } from "./normalizeAdminWalletRouteError";
import { useAdminWalletsData } from "./useAdminWalletsData";
import { useWalletCreditsCsvDownload } from "./useWalletCreditsCsvDownload";

function isExpiredSession(error: RouteError) {
  return error.status === 401 || error.status === 404;
}

export function useAdminWalletsRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const workspace = useMemo(
    () => (session ? buildPlatformAdminWorkspace(session) : emptyPlatformWalletWorkspace),
    [session],
  );
  const allowed = session ? hasPlatformAdminAccess(session) : false;
  const canView = canViewWalletAudit(workspace);
  const canExport = canExportWalletCredits(workspace);
  const walletCapability = findPlatformCapability(workspace, "wallets");
  const csv = useWalletCreditsCsvDownload({ canExport });
  const data = useAdminWalletsData({ allowed, canView, session });
  const { loadWalletAudit } = data;

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
      setSession(await loadAdminWalletSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeAdminWalletRouteError(
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
    if (session && allowed && canView) {
      const timeout = window.setTimeout(() => loadWalletAudit(), 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [allowed, canView, loadWalletAudit, session]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    allowed,
    canExport,
    canView,
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
    walletCapability,
    workspace,
    ...csv,
    ...data,
  };
}

export type AdminWalletsRouteController = ReturnType<typeof useAdminWalletsRoute>;
